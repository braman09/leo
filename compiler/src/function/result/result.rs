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

//! Enforces that one return value is produced in a compiled Leo program.

use crate::{
    errors::StatementError,
    get_indicator_value,
    program::ConstrainedProgram,
    value::ConstrainedValue,
    GroupType,
};

use leo_asg::{Span, Type};

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{boolean::Boolean, select::CondSelectGadget},
    },
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    ///
    /// Returns a conditionally selected result from the given possible function returns and
    /// given function return type.
    ///
    pub fn conditionally_select_result<CS: ConstraintSystem<F>>(
        cs: &mut CS,
        expected_return: &Type,
        results: Vec<(Boolean, ConstrainedValue<F, G>)>,
        span: &Span,
    ) -> Result<ConstrainedValue<F, G>, StatementError> {
        // Initialize empty return value.
        let mut return_value = ConstrainedValue::Tuple(vec![]);

        // Error if the function or one of its branches does not return.
        if !expected_return.is_unit()
            && results
                .iter()
                .find(|(indicator, _res)| get_indicator_value(indicator))
                .is_none()
        {
            return Err(StatementError::no_returns(&expected_return, span.to_owned()));
        }

        // Find the return value
        let mut ignored = vec![];
        let mut found_return = false;
        for (indicator, result) in results.into_iter() {
            // Error if a statement returned a result with an incorrect type
            let result_type = result.to_type(span)?;
            if !expected_return.is_assignable_from(&result_type) {
                panic!(
                    "failed type resolution for function return: expected '{}', got '{}'",
                    expected_return.to_string(),
                    result_type.to_string()
                );
            }

            if get_indicator_value(&indicator) {
                // Error if we already have a return value.
                if found_return {
                    return Err(StatementError::multiple_returns(span.to_owned()));
                } else {
                    // Set the function return value.
                    return_value = result;
                    found_return = true;
                }
            } else {
                // Ignore a possible function return value.
                ignored.push((indicator, result))
            }
        }

        // Conditionally select out the ignored results in the circuit.
        //
        // If there are branches in the function we need to use the `ConditionalSelectGadget` to parse through and select the correct one.
        // This can be thought of as de-multiplexing all previous wires that may have returned results into one.
        for (i, (indicator, result)) in ignored.into_iter().enumerate() {
            return_value = ConstrainedValue::conditionally_select(
                cs.ns(|| format!("select result {} {}:{}", i, span.line, span.start)),
                &indicator,
                &result,
                &return_value,
            )
            .map_err(|_| StatementError::select_fail(result.to_string(), return_value.to_string(), span.to_owned()))?;
        }

        Ok(return_value)
    }
}
