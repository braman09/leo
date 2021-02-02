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

use crate::{AsgConvertError, PartialType, Scope, Span};

/// A node in the abstract semantic graph.
pub trait Node {
    fn span(&self) -> Option<&Span>;
}

pub(super) trait FromAst<T: leo_ast::Node + 'static>: Sized + 'static {
    // expected_type contract: if present, output expression must be of type expected_type.
    // type of an element may NEVER be None unless it is functionally a non-expression. (static call targets, function ref call targets are not expressions)
    fn from_ast(scope: &Scope, value: &T, expected_type: Option<PartialType>) -> Result<Self, AsgConvertError>;
}
