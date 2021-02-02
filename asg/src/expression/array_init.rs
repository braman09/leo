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

use std::{
    cell::RefCell,
    sync::{Arc, Weak},
};

pub struct ArrayInitExpression {
    pub parent: RefCell<Option<Weak<Expression>>>,
    pub span: Option<Span>,
    pub element: Arc<Expression>,
    pub len: usize,
}

impl Node for ArrayInitExpression {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl ExpressionNode for ArrayInitExpression {
    fn set_parent(&self, parent: Weak<Expression>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<Arc<Expression>> {
        self.parent.borrow().as_ref().map(Weak::upgrade).flatten()
    }

    fn enforce_parents(&self, expr: &Arc<Expression>) {
        self.element.set_parent(Arc::downgrade(expr));
    }

    fn get_type(&self) -> Option<Type> {
        Some(Type::Array(Box::new(self.element.get_type()?), self.len))
    }

    fn is_mut_ref(&self) -> bool {
        false
    }

    fn const_value(&self) -> Option<ConstValue> {
        // not implemented due to performance concerns
        None
    }

    fn is_consty(&self) -> bool {
        self.element.is_consty()
    }
}

impl FromAst<leo_ast::ArrayInitExpression> for ArrayInitExpression {
    fn from_ast(
        scope: &Scope,
        value: &leo_ast::ArrayInitExpression,
        expected_type: Option<PartialType>,
    ) -> Result<ArrayInitExpression, AsgConvertError> {
        let (mut expected_item, expected_len) = match expected_type {
            Some(PartialType::Array(item, dims)) => (item.map(|x| *x), dims),
            None => (None, None),
            Some(type_) => {
                return Err(AsgConvertError::unexpected_type(
                    &type_.to_string(),
                    Some("array"),
                    &value.span,
                ));
            }
        };
        let dimensions = value
            .dimensions
            .0
            .iter()
            .map(|x| {
                x.value
                    .parse::<usize>()
                    .map_err(|_| AsgConvertError::parse_dimension_error())
            })
            .collect::<Result<Vec<_>, AsgConvertError>>()?;

        let len = *dimensions.get(0).ok_or_else(AsgConvertError::parse_dimension_error)?;
        if let Some(expected_len) = expected_len {
            if expected_len != len {
                return Err(AsgConvertError::unexpected_type(
                    &*format!("array of length {}", expected_len),
                    Some(&*format!("array of length {}", len)),
                    &value.span,
                ));
            }
        }

        for dimension in (&dimensions[1..]).iter().copied() {
            expected_item = match expected_item {
                Some(PartialType::Array(item, len)) => {
                    if let Some(len) = len {
                        if len != dimension {
                            return Err(AsgConvertError::unexpected_type(
                                &*format!("array of length {}", dimension),
                                Some(&*format!("array of length {}", len)),
                                &value.span,
                            ));
                        }
                    }

                    item.map(|x| *x)
                }
                None => None,
                Some(type_) => {
                    return Err(AsgConvertError::unexpected_type(
                        "array",
                        Some(&type_.to_string()),
                        &value.span,
                    ));
                }
            }
        }
        let mut element = Some(Arc::<Expression>::from_ast(scope, &*value.element, expected_item)?);
        let mut output = None;

        for dimension in dimensions.iter().rev().copied() {
            output = Some(ArrayInitExpression {
                parent: RefCell::new(None),
                span: Some(value.span.clone()),
                element: output
                    .map(Expression::ArrayInit)
                    .map(Arc::new)
                    .unwrap_or_else(|| element.take().unwrap()),
                len: dimension,
            });
        }
        Ok(output.unwrap())
    }
}

impl Into<leo_ast::ArrayInitExpression> for &ArrayInitExpression {
    fn into(self) -> leo_ast::ArrayInitExpression {
        leo_ast::ArrayInitExpression {
            element: Box::new(self.element.as_ref().into()),
            dimensions: leo_ast::ArrayDimensions(vec![leo_ast::PositiveNumber {
                value: self.len.to_string(),
            }]),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
