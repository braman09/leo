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

//! An in memory store to keep track of defined names when constraining a Leo program.

use crate::{value::ConstrainedValue, GroupType};

use leo_asg::Program;
use snarkvm_models::curves::{Field, PrimeField};

use indexmap::IndexMap;
use uuid::Uuid;

pub struct ConstrainedProgram<F: Field + PrimeField, G: GroupType<F>> {
    pub asg: Program,
    identifiers: IndexMap<Uuid, ConstrainedValue<F, G>>,
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn new(asg: Program) -> Self {
        Self {
            asg,
            identifiers: IndexMap::new(),
        }
    }

    pub(crate) fn store(&mut self, name: Uuid, value: ConstrainedValue<F, G>) {
        self.identifiers.insert(name, value);
    }

    pub(crate) fn get(&self, name: &Uuid) -> Option<&ConstrainedValue<F, G>> {
        self.identifiers.get(name)
    }

    pub(crate) fn get_mut(&mut self, name: &Uuid) -> Option<&mut ConstrainedValue<F, G>> {
        self.identifiers.get_mut(name)
    }
}
