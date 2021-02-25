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

use leo_ast::{FormattedError, LeoError, Span};

use crate::{DeprecatedError, Token, TokenError};

#[derive(Debug, Error)]
pub enum SyntaxError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    TokenError(#[from] TokenError),

    #[error("{}", _0)]
    DeprecatedError(#[from] DeprecatedError),
}

impl LeoError for SyntaxError {
    fn get_path(&self) -> Option<&str> {
        match self {
            SyntaxError::Error(error) => error.get_path(),
            SyntaxError::TokenError(error) => error.get_path(),
            SyntaxError::DeprecatedError(error) => error.get_path(),
        }
    }

    fn set_path(&mut self, path: &str, contents: &[String]) {
        match self {
            SyntaxError::Error(error) => error.set_path(path, contents),
            SyntaxError::TokenError(error) => error.set_path(path, contents),
            SyntaxError::DeprecatedError(error) => error.set_path(path, contents),
        }
    }
}

impl SyntaxError {
    fn new_from_span(message: String, span: &Span) -> Self {
        SyntaxError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn unexpected_eof(span: &Span) -> Self {
        Self::new_from_span("unexpected EOF".to_string(), span)
    }

    pub fn unexpected(got: &Token, expected: &[Token], span: &Span) -> Self {
        Self::new_from_span(
            format!(
                "expected {} -- got '{}'",
                expected
                    .iter()
                    .map(|x| format!("'{}'", x))
                    .collect::<Vec<_>>()
                    .join(", "),
                got.to_string()
            ),
            span,
        )
    }

    pub fn unexpected_ident(got: &str, expected: &[&str], span: &Span) -> Self {
        Self::new_from_span(
            format!(
                "expected identifier {} -- got '{}'",
                expected
                    .iter()
                    .map(|x| format!("'{}'", x))
                    .collect::<Vec<_>>()
                    .join(", "),
                got
            ),
            span,
        )
    }

    pub fn unexpected_str(got: &Token, expected: &str, span: &Span) -> Self {
        Self::new_from_span(format!("expected '{}', got '{}'", expected, got.to_string()), span)
    }

    pub fn spread_in_array_init(span: &Span) -> Self {
        Self::new_from_span("illegal spread in array initializer".to_string(), span)
    }

    pub fn invalid_assignment_target(span: &Span) -> Self {
        Self::new_from_span("invalid assignment target".to_string(), span)
    }

    pub fn invalid_package_name(span: &Span) -> Self {
        Self::new_from_span(
            "package names must be lowercase alphanumeric ascii with underscores and singular dashes".to_string(),
            span,
        )
    }
}
