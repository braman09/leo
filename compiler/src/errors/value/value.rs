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

use crate::errors::{AddressError, BooleanError, FieldError, GroupError, IntegerError};
use leo_ast::{FormattedError, LeoError, Span};

#[derive(Debug, Error)]
pub enum ValueError {
    #[error("{}", _0)]
    AddressError(#[from] AddressError),

    #[error("{}", _0)]
    BooleanError(#[from] BooleanError),

    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    FieldError(#[from] FieldError),

    #[error("{}", _0)]
    GroupError(#[from] GroupError),

    #[error("{}", _0)]
    IntegerError(#[from] IntegerError),
}

impl LeoError for ValueError {
    fn get_path(&self) -> Option<&str> {
        match self {
            ValueError::AddressError(error) => error.get_path(),
            ValueError::BooleanError(error) => error.get_path(),
            ValueError::Error(error) => error.get_path(),
            ValueError::FieldError(error) => error.get_path(),
            ValueError::GroupError(error) => error.get_path(),
            ValueError::IntegerError(error) => error.get_path(),
        }
    }

    fn set_path(&mut self, path: &str, contents: &[String]) {
        match self {
            ValueError::AddressError(error) => error.set_path(path, contents),
            ValueError::BooleanError(error) => error.set_path(path, contents),
            ValueError::Error(error) => error.set_path(path, contents),
            ValueError::FieldError(error) => error.set_path(path, contents),
            ValueError::GroupError(error) => error.set_path(path, contents),
            ValueError::IntegerError(error) => error.set_path(path, contents),
        }
    }
}

impl ValueError {
    fn new_from_span(message: String, span: &Span) -> Self {
        ValueError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn implicit(value: String, span: &Span) -> Self {
        let message = format!("explicit type needed for `{}`", value);

        Self::new_from_span(message, span)
    }

    pub fn implicit_group(span: &Span) -> Self {
        let message = "group coordinates should be in (x, y)group format".to_string();

        Self::new_from_span(message, span)
    }
}
