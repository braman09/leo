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

//! Enforce constraints on an expression in a compiled Leo program.

use crate::{
    arithmetic::*,
    errors::ExpressionError,
    logical::*,
    program::ConstrainedProgram,
    relational::*,
    value::{Address, ConstrainedValue, Integer},
    FieldType,
    GroupType,
};
use leo_asg::{expression::*, ConstValue, Expression, Node};
use std::sync::Arc;

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub(crate) fn enforce_expression<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        expression: &Arc<Expression>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        let span = expression.span().cloned().unwrap_or_default();
        match &**expression {
            // Variables
            Expression::VariableRef(variable_ref) => self.evaluate_ref(file_scope, function_scope, variable_ref),

            // Values
            Expression::Constant(Constant { value, .. }) => {
                Ok(match value {
                    ConstValue::Address(value) => ConstrainedValue::Address(Address::constant(value.clone(), &span)?),
                    ConstValue::Boolean(value) => ConstrainedValue::Boolean(Boolean::Constant(*value)),
                    ConstValue::Field(value) => ConstrainedValue::Field(FieldType::constant(value.to_string(), &span)?),
                    ConstValue::Group(value) => ConstrainedValue::Group(G::constant(value, &span)?),
                    ConstValue::Int(value) => ConstrainedValue::Integer(Integer::new(value, &span)),
                    ConstValue::Tuple(_) | ConstValue::Array(_) => unimplemented!(), // shouldnt be in the asg here
                })
            }

            // Binary operations
            Expression::Binary(BinaryExpression {
                left, right, operation, ..
            }) => {
                let (resolved_left, resolved_right) =
                    self.enforce_binary_expression(cs, file_scope, function_scope, left, right, &span)?;

                match operation {
                    BinaryOperation::Add => enforce_add(cs, resolved_left, resolved_right, &span),
                    BinaryOperation::Sub => enforce_sub(cs, resolved_left, resolved_right, &span),
                    BinaryOperation::Mul => enforce_mul(cs, resolved_left, resolved_right, &span),
                    BinaryOperation::Div => enforce_div(cs, resolved_left, resolved_right, &span),
                    BinaryOperation::Pow => enforce_pow(cs, resolved_left, resolved_right, &span),
                    BinaryOperation::Or => {
                        enforce_or(cs, resolved_left, resolved_right, &span).map_err(ExpressionError::BooleanError)
                    }
                    BinaryOperation::And => {
                        enforce_and(cs, resolved_left, resolved_right, &span).map_err(ExpressionError::BooleanError)
                    }
                    BinaryOperation::Eq => evaluate_eq(cs, resolved_left, resolved_right, &span),
                    BinaryOperation::Ne => evaluate_not(evaluate_eq(cs, resolved_left, resolved_right, &span)?, &span)
                        .map_err(ExpressionError::BooleanError),
                    BinaryOperation::Ge => evaluate_ge(cs, resolved_left, resolved_right, &span),
                    BinaryOperation::Gt => evaluate_gt(cs, resolved_left, resolved_right, &span),
                    BinaryOperation::Le => evaluate_le(cs, resolved_left, resolved_right, &span),
                    BinaryOperation::Lt => evaluate_lt(cs, resolved_left, resolved_right, &span),
                }
            }

            // Unary operations
            Expression::Unary(UnaryExpression { inner, operation, .. }) => match operation {
                UnaryOperation::Negate => {
                    let resolved_inner = self.enforce_expression(cs, file_scope, function_scope, inner)?;
                    enforce_negate(cs, resolved_inner, &span)
                }
                UnaryOperation::Not => Ok(evaluate_not(
                    self.enforce_operand(cs, file_scope, function_scope, inner)?,
                    &span,
                )?),
            },

            Expression::Conditional(ConditionalExpression {
                condition,
                if_true,
                if_false,
                ..
            }) => {
                self.enforce_conditional_expression(cs, file_scope, function_scope, condition, if_true, if_false, &span)
            }

            // Arrays
            Expression::ArrayInline(ArrayInlineExpression { elements, .. }) => {
                self.enforce_array(cs, file_scope, function_scope, elements, span)
            }
            Expression::ArrayInit(ArrayInitExpression { element, len, .. }) => {
                self.enforce_array_initializer(cs, file_scope, function_scope, element, *len, span)
            }
            Expression::ArrayAccess(ArrayAccessExpression { array, index, .. }) => {
                self.enforce_array_access(cs, file_scope, function_scope, array, index, &span)
            }
            Expression::ArrayRangeAccess(ArrayRangeAccessExpression { array, left, right, .. }) => self
                .enforce_array_range_access(
                    cs,
                    file_scope,
                    function_scope,
                    array,
                    left.as_ref(),
                    right.as_ref(),
                    &span,
                ),

            // Tuples
            Expression::TupleInit(TupleInitExpression { elements, .. }) => {
                self.enforce_tuple(cs, file_scope, function_scope, elements, &span)
            }
            Expression::TupleAccess(TupleAccessExpression { tuple_ref, index, .. }) => {
                self.enforce_tuple_access(cs, file_scope, function_scope, tuple_ref, *index, &span)
            }

            // Circuits
            Expression::CircuitInit(expr) => self.enforce_circuit(cs, file_scope, function_scope, expr, &span),
            Expression::CircuitAccess(expr) => self.enforce_circuit_access(cs, file_scope, function_scope, expr, &span),

            // Functions
            Expression::Call(CallExpression {
                function,
                target,
                arguments,
                ..
            }) => match function {
                function if function.name.borrow().is_core() => self.enforce_core_circuit_call_expression(
                    cs,
                    file_scope,
                    function_scope,
                    function.name.borrow().name.clone(),
                    arguments,
                    &span,
                ),
                function => self.enforce_function_call_expression(
                    cs,
                    file_scope,
                    function_scope,
                    &function,
                    target.as_ref(),
                    arguments,
                    &span,
                ),
            },
        }
    }
}
