// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{AsgConvertError, ConstValue, Expression, ExpressionNode, FromAst, Node, PartialType, Scope, Span, Type};
use leo_ast::IntegerType;

use std::{
    cell::RefCell,
    sync::{Arc, Weak},
};

pub struct ArrayRangeAccessExpression {
    pub parent: RefCell<Option<Weak<Expression>>>,
    pub span: Option<Span>,
    pub array: Arc<Expression>,
    pub left: Option<Arc<Expression>>,
    pub right: Option<Arc<Expression>>,
}

impl Node for ArrayRangeAccessExpression {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl ExpressionNode for ArrayRangeAccessExpression {
    fn set_parent(&self, parent: Weak<Expression>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<Arc<Expression>> {
        self.parent.borrow().as_ref().map(Weak::upgrade).flatten()
    }

    fn enforce_parents(&self, expr: &Arc<Expression>) {
        self.array.set_parent(Arc::downgrade(expr));
        self.array.enforce_parents(&self.array);
        if let Some(left) = self.left.as_ref() {
            left.set_parent(Arc::downgrade(expr));
        }
        if let Some(right) = self.right.as_ref() {
            right.set_parent(Arc::downgrade(expr));
        }
    }

    fn get_type(&self) -> Option<Type> {
        let (element, array_len) = match self.array.get_type() {
            Some(Type::Array(element, len)) => (element, len),
            _ => return None,
        };
        let const_left = match self.left.as_ref().map(|x| x.const_value()) {
            Some(Some(ConstValue::Int(x))) => x.to_usize()?,
            None => 0,
            _ => return None,
        };
        let const_right = match self.right.as_ref().map(|x| x.const_value()) {
            Some(Some(ConstValue::Int(x))) => x.to_usize()?,
            None => array_len,
            _ => return None,
        };
        if const_left > const_right || const_right > array_len {
            return None;
        }

        Some(Type::Array(element, const_right - const_left))
    }

    fn is_mut_ref(&self) -> bool {
        self.array.is_mut_ref()
    }

    fn const_value(&self) -> Option<ConstValue> {
        let mut array = match self.array.const_value()? {
            ConstValue::Array(values) => values,
            _ => return None,
        };
        let const_left = match self.left.as_ref().map(|x| x.const_value()) {
            Some(Some(ConstValue::Int(x))) => x.to_usize()?,
            None => 0,
            _ => return None,
        };
        let const_right = match self.right.as_ref().map(|x| x.const_value()) {
            Some(Some(ConstValue::Int(x))) => x.to_usize()?,
            None => array.len(),
            _ => return None,
        };
        if const_left > const_right || const_right as usize > array.len() {
            return None;
        }

        Some(ConstValue::Array(array.drain(const_left..const_right).collect()))
    }

    fn is_consty(&self) -> bool {
        self.array.is_consty()
    }
}

impl FromAst<leo_ast::ArrayRangeAccessExpression> for ArrayRangeAccessExpression {
    fn from_ast(
        scope: &Scope,
        value: &leo_ast::ArrayRangeAccessExpression,
        expected_type: Option<PartialType>,
    ) -> Result<ArrayRangeAccessExpression, AsgConvertError> {
        let expected_array = match expected_type {
            Some(PartialType::Array(element, _len)) => Some(PartialType::Array(element, None)),
            None => None,
            Some(x) => {
                return Err(AsgConvertError::unexpected_type(
                    &x.to_string(),
                    Some("array"),
                    &value.span,
                ));
            }
        };
        let array = Arc::<Expression>::from_ast(scope, &*value.array, expected_array)?;
        let array_type = array.get_type();
        match array_type {
            Some(Type::Array(_, _)) => (),
            type_ => {
                return Err(AsgConvertError::unexpected_type(
                    "array",
                    type_.map(|x| x.to_string()).as_deref(),
                    &value.span,
                ));
            }
        }
        let left = value
            .left
            .as_deref()
            .map(|left| {
                Arc::<Expression>::from_ast(scope, left, Some(PartialType::Integer(None, Some(IntegerType::U32))))
            })
            .transpose()?;
        let right = value
            .right
            .as_deref()
            .map(|right| {
                Arc::<Expression>::from_ast(scope, right, Some(PartialType::Integer(None, Some(IntegerType::U32))))
            })
            .transpose()?;

        if let Some(left) = left.as_ref() {
            if !left.is_consty() {
                return Err(AsgConvertError::unexpected_nonconst(
                    &left.span().cloned().unwrap_or_default(),
                ));
            }
        }
        if let Some(right) = right.as_ref() {
            if !right.is_consty() {
                return Err(AsgConvertError::unexpected_nonconst(
                    &right.span().cloned().unwrap_or_default(),
                ));
            }
        }
        Ok(ArrayRangeAccessExpression {
            parent: RefCell::new(None),
            span: Some(value.span.clone()),
            array,
            left,
            right,
        })
    }
}

impl Into<leo_ast::ArrayRangeAccessExpression> for &ArrayRangeAccessExpression {
    fn into(self) -> leo_ast::ArrayRangeAccessExpression {
        leo_ast::ArrayRangeAccessExpression {
            array: Box::new(self.array.as_ref().into()),
            left: self.left.as_ref().map(|left| Box::new(left.as_ref().into())),
            right: self.right.as_ref().map(|right| Box::new(right.as_ref().into())),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
