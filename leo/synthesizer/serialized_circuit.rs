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

use crate::synthesizer::{CircuitSynthesizer, SerializedField, SerializedIndex};
use leo_compiler::Output;

use snarkos_curves::bls12_377::Bls12_377;
use snarkos_errors::curves::FieldError;
use snarkos_models::{
    curves::PairingEngine,
    gadgets::r1cs::{ConstraintSystem, Index},
};

use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Serialize, Deserialize)]
pub struct SerializedCircuit {
    pub num_inputs: usize,
    pub num_aux: usize,
    pub num_constraints: usize,

    pub input_assignment: Vec<SerializedField>,
    pub aux_assignment: Vec<SerializedField>,
    pub program_input_range: (usize, usize),
    pub output_indices: Vec<usize>,

    pub at: Vec<Vec<(SerializedField, SerializedIndex)>>,
    pub bt: Vec<Vec<(SerializedField, SerializedIndex)>>,
    pub ct: Vec<Vec<(SerializedField, SerializedIndex)>>,
}

impl SerializedCircuit {
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        Ok(serde_json::to_string_pretty(&self)?)
    }

    pub fn from_json_string(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn new<E: PairingEngine>(synthesizer: CircuitSynthesizer<E>, output: Output) -> Self {
        let num_inputs = synthesizer.input_assignment.len();
        let num_aux = synthesizer.aux_assignment.len();
        let num_constraints = synthesizer.num_constraints();

        // Serialize assignments
        fn get_serialized_assignments<E: PairingEngine>(assignments: &[E::Fr]) -> Vec<SerializedField> {
            assignments.iter().map(SerializedField::from).collect()
        }

        // Serialize input assignments.
        let input_assignment = get_serialized_assignments::<E>(&synthesizer.input_assignment);

        // Serialize aux assignments.
        let aux_assignment = get_serialized_assignments::<E>(&synthesizer.aux_assignment);

        // Serialize program input indices.
        let program_input_range = output.input_range();

        // Serialize output indices.
        let output_indices = output.output_indices();

        // Serialize constraints.
        fn get_serialized_constraints<E: PairingEngine>(
            constraints: &[(E::Fr, Index)],
        ) -> Vec<(SerializedField, SerializedIndex)> {
            let mut serialized = Vec::with_capacity(constraints.len());

            for &(ref coeff, index) in constraints {
                let field = SerializedField::from(coeff);
                let index = SerializedIndex::from(index);

                serialized.push((field, index))
            }

            serialized
        }

        let mut at = Vec::with_capacity(num_constraints);
        let mut bt = Vec::with_capacity(num_constraints);
        let mut ct = Vec::with_capacity(num_constraints);

        for i in 0..num_constraints {
            // Serialize at[i].

            let a_constraints = get_serialized_constraints::<E>(&synthesizer.at[i]);
            at.push(a_constraints);

            // Serialize bt[i].
            let b_constraints = get_serialized_constraints::<E>(&synthesizer.bt[i]);
            bt.push(b_constraints);

            // Serialize ct[i].
            let c_constraints = get_serialized_constraints::<E>(&synthesizer.ct[i]);
            ct.push(c_constraints);
        }

        Self {
            num_inputs,
            num_aux,
            num_constraints,
            input_assignment,
            aux_assignment,
            program_input_range,
            output_indices,
            at,
            bt,
            ct,
        }
    }
}

impl TryFrom<SerializedCircuit> for CircuitSynthesizer<Bls12_377> {
    type Error = FieldError;

    fn try_from(serialized: SerializedCircuit) -> Result<CircuitSynthesizer<Bls12_377>, Self::Error> {
        // Deserialize assignments
        fn get_deserialized_assignments(
            assignments: &[SerializedField],
        ) -> Result<Vec<<Bls12_377 as PairingEngine>::Fr>, FieldError> {
            let mut deserialized = Vec::with_capacity(assignments.len());

            for serialized_assignment in assignments {
                let field = <Bls12_377 as PairingEngine>::Fr::try_from(serialized_assignment)?;

                deserialized.push(field);
            }

            Ok(deserialized)
        }

        let input_assignment = get_deserialized_assignments(&serialized.input_assignment)?;
        let aux_assignment = get_deserialized_assignments(&serialized.aux_assignment)?;

        // Deserialize constraints
        fn get_deserialized_constraints(
            constraints: &[(SerializedField, SerializedIndex)],
        ) -> Result<Vec<(<Bls12_377 as PairingEngine>::Fr, Index)>, FieldError> {
            let mut deserialized = Vec::with_capacity(constraints.len());

            for &(ref serialized_coeff, ref serialized_index) in constraints {
                let field = <Bls12_377 as PairingEngine>::Fr::try_from(serialized_coeff)?;
                let index = Index::from(serialized_index);

                deserialized.push((field, index));
            }

            Ok(deserialized)
        }

        let mut at = Vec::with_capacity(serialized.num_constraints);
        let mut bt = Vec::with_capacity(serialized.num_constraints);
        let mut ct = Vec::with_capacity(serialized.num_constraints);

        for i in 0..serialized.num_constraints {
            // Deserialize at[i]

            let a_constraints = get_deserialized_constraints(&serialized.at[i])?;
            at.push(a_constraints);

            // Deserialize bt[i]

            let b_constraints = get_deserialized_constraints(&serialized.bt[i])?;
            bt.push(b_constraints);

            // Deserialize ct[i]

            let c_constraints = get_deserialized_constraints(&serialized.ct[i])?;
            ct.push(c_constraints);
        }

        Ok(CircuitSynthesizer::<Bls12_377> {
            input_assignment,
            aux_assignment,
            at,
            bt,
            ct,
        })
    }
}
