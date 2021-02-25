// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::{Function, Identifier, Type};

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitMember {
    // (variable_name, variable_type)
    CircuitVariable(Identifier, Type),
    // (function)
    CircuitFunction(Function),
}

impl fmt::Display for CircuitMember {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CircuitMember::CircuitVariable(ref identifier, ref type_) => write!(f, "{}: {}", identifier, type_),
            CircuitMember::CircuitFunction(ref function) => write!(f, "{}", function),
        }
    }
}
