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

use super::*;

const ASSIGN_TOKENS: &[Token] = &[
    Token::Assign,
    Token::AddEq,
    Token::MinusEq,
    Token::MulEq,
    Token::DivEq,
    Token::ExpEq,
    Token::BitAndEq,
    Token::BitOrEq,
    Token::BitXorEq,
    Token::ShlEq,
    Token::ShrEq,
    Token::ShrSignedEq,
    Token::ModEq,
    Token::OrEq,
    Token::AndEq,
];

impl ParserContext {
    pub fn construct_assignee_access(expr: Expression, accesses: &mut Vec<AssigneeAccess>) -> SyntaxResult<Identifier> {
        let identifier;
        match expr {
            Expression::CircuitMemberAccess(expr) => {
                identifier = Self::construct_assignee_access(*expr.circuit, accesses)?;
                accesses.push(AssigneeAccess::Member(expr.name));
            }
            Expression::TupleAccess(expr) => {
                identifier = Self::construct_assignee_access(*expr.tuple, accesses)?;
                accesses.push(AssigneeAccess::Tuple(expr.index, expr.span));
            }
            Expression::ArrayRangeAccess(expr) => {
                identifier = Self::construct_assignee_access(*expr.array, accesses)?;
                accesses.push(AssigneeAccess::ArrayRange(
                    expr.left.map(|x| *x),
                    expr.right.map(|x| *x),
                ));
            }
            Expression::ArrayAccess(expr) => {
                identifier = Self::construct_assignee_access(*expr.array, accesses)?;
                accesses.push(AssigneeAccess::ArrayIndex(*expr.index));
            }
            Expression::Identifier(id) => identifier = id,
            _ => return Err(SyntaxError::invalid_assignment_target(expr.span())),
        }
        Ok(identifier)
    }

    pub fn construct_assignee(expr: Expression) -> SyntaxResult<Assignee> {
        let expr_span = expr.span().clone();
        let mut accesses = vec![];
        let identifier = Self::construct_assignee_access(expr, &mut accesses)?;

        Ok(Assignee {
            span: expr_span,
            identifier,
            accesses,
        })
    }

    pub fn parse_statement(&mut self) -> SyntaxResult<Statement> {
        match &self.peek()?.token {
            Token::Return => Ok(Statement::Return(self.parse_return_statement()?)),
            Token::If => Ok(Statement::Conditional(self.parse_conditional_statement()?)),
            Token::For => Ok(Statement::Iteration(self.parse_for_statement()?)),
            Token::Console => Ok(Statement::Console(self.parse_console_statement()?)),
            Token::Let | Token::Const => Ok(Statement::Definition(self.parse_definition_statement()?)),
            Token::LeftCurly => Ok(Statement::Block(self.parse_block()?)),
            _ => {
                let expr = self.parse_expression()?;

                if let Some(operator) = self.eat_any(ASSIGN_TOKENS) {
                    let value = self.parse_expression()?;
                    let assignee = Self::construct_assignee(expr)?;
                    self.expect(Token::Semicolon)?;
                    Ok(Statement::Assign(AssignStatement {
                        span: &assignee.span + value.span(),
                        assignee,
                        operation: match operator.token {
                            Token::Assign => AssignOperation::Assign,
                            Token::AddEq => AssignOperation::Add,
                            Token::MinusEq => AssignOperation::Sub,
                            Token::MulEq => AssignOperation::Mul,
                            Token::DivEq => AssignOperation::Div,
                            Token::ExpEq => AssignOperation::Pow,
                            Token::OrEq => AssignOperation::Or,
                            Token::AndEq => AssignOperation::And,
                            Token::BitOrEq => AssignOperation::BitOr,
                            Token::BitAndEq => AssignOperation::BitAnd,
                            Token::BitXorEq => AssignOperation::BitXor,
                            Token::ShrEq => AssignOperation::Shr,
                            Token::ShrSignedEq => AssignOperation::ShrSigned,
                            Token::ShlEq => AssignOperation::Shl,
                            Token::ModEq => AssignOperation::Mod,
                            _ => unimplemented!(),
                        },
                        value,
                    }))
                } else {
                    self.expect(Token::Semicolon)?;
                    Ok(Statement::Expression(ExpressionStatement {
                        span: expr.span().clone(),
                        expression: expr,
                    }))
                }
            }
        }
    }

    pub fn parse_block(&mut self) -> SyntaxResult<Block> {
        let start = self.expect(Token::LeftCurly)?;
        let mut statements = vec![];
        loop {
            match self.eat(Token::RightCurly) {
                None => {
                    statements.push(self.parse_statement()?);
                }
                Some(end) => {
                    return Ok(Block {
                        span: start + end.span,
                        statements,
                    });
                }
            }
        }
    }

    pub fn parse_return_statement(&mut self) -> SyntaxResult<ReturnStatement> {
        let start = self.expect(Token::Return)?;
        let expr = self.parse_expression()?;
        self.eat(Token::Comma);

        Ok(ReturnStatement {
            span: &start + expr.span(),
            expression: expr,
        })
    }

    pub fn parse_conditional_statement(&mut self) -> SyntaxResult<ConditionalStatement> {
        let start = self.expect(Token::If)?;
        self.fuzzy_struct_state = true;
        let expr = self.parse_expression_fuzzy()?;
        self.fuzzy_struct_state = false;
        let body = self.parse_block()?;
        let next = if self.eat(Token::Else).is_some() {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Ok(ConditionalStatement {
            span: &start + next.as_ref().map(|x| x.span()).unwrap_or(&body.span),
            condition: expr,
            block: body,
            next,
        })
    }

    pub fn parse_for_statement(&mut self) -> SyntaxResult<IterationStatement> {
        let start_span = self.expect(Token::For)?;
        let ident = self.expect_ident()?;
        self.expect(Token::In)?;
        let start = self.parse_expression()?;
        self.expect(Token::DotDot)?;
        self.fuzzy_struct_state = true;
        let stop = self.parse_expression_fuzzy()?;
        self.fuzzy_struct_state = false;
        let block = self.parse_block()?;

        Ok(IterationStatement {
            span: start_span + block.span.clone(),
            variable: ident,
            start,
            stop,
            block,
        })
    }

    pub fn parse_formatted_string(&mut self) -> SyntaxResult<FormattedString> {
        let start_span;
        let parts = match self.expect_any()? {
            SpannedToken {
                token: Token::FormattedString(parts),
                span,
            } => {
                start_span = span;
                parts
            }
            SpannedToken { token, span } => return Err(SyntaxError::unexpected_str(&token, "formatted string", &span)),
        };
        let mut parameters = vec![];
        while self.eat(Token::Comma).is_some() {
            let param = self.parse_expression()?;
            parameters.push(param);
        }

        Ok(FormattedString {
            parts: parts
                .into_iter()
                .map(|x| match x {
                    crate::FormattedStringPart::Const(value) => FormattedStringPart::Const(value),
                    crate::FormattedStringPart::Container => FormattedStringPart::Container,
                })
                .collect(),
            span: &start_span + parameters.last().map(|x| x.span()).unwrap_or(&start_span),
            parameters,
        })
    }

    pub fn parse_console_statement(&mut self) -> SyntaxResult<ConsoleStatement> {
        let keyword = self.expect(Token::Console)?;
        self.expect(Token::Dot)?;
        let function = self.expect_ident()?;
        self.expect(Token::LeftParen)?;
        let function = match &*function.name {
            "assert" => {
                let expr = self.parse_expression()?;
                ConsoleFunction::Assert(expr)
            }
            "debug" => ConsoleFunction::Debug(self.parse_formatted_string()?),
            "error" => ConsoleFunction::Error(self.parse_formatted_string()?),
            "log" => ConsoleFunction::Log(self.parse_formatted_string()?),
            x => {
                return Err(SyntaxError::unexpected_ident(
                    &x,
                    &["assert", "debug", "error", "log"],
                    &function.span,
                ));
            }
        };
        self.expect(Token::RightParen)?;
        self.expect(Token::Semicolon)?;

        Ok(ConsoleStatement {
            span: &keyword + function.span(),
            function,
        })
    }

    pub fn parse_variable_name(&mut self) -> SyntaxResult<VariableName> {
        let mutable = self.eat(Token::Mut);
        let name = self.expect_ident()?;
        Ok(VariableName {
            span: mutable
                .as_ref()
                .map(|x| &x.span + &name.span)
                .unwrap_or_else(|| name.span.clone()),
            mutable: mutable.is_some(),
            identifier: name,
        })
    }

    pub fn parse_definition_statement(&mut self) -> SyntaxResult<DefinitionStatement> {
        let declare = self.expect_oneof(&[Token::Let, Token::Const])?;
        let mut variable_names = vec![];
        if self.eat(Token::LeftParen).is_some() {
            variable_names.push(self.parse_variable_name()?);
            while self.eat(Token::Comma).is_some() {
                variable_names.push(self.parse_variable_name()?);
            }
            self.expect(Token::RightParen)?;
        } else {
            variable_names.push(self.parse_variable_name()?);
        }

        let type_ = if self.eat(Token::Colon).is_some() {
            Some(self.parse_type()?.0)
        } else {
            None
        };

        self.expect(Token::Assign)?;
        let expr = self.parse_expression()?;
        self.expect(Token::Semicolon)?;

        Ok(DefinitionStatement {
            span: &declare.span + expr.span(),
            declaration_type: match declare.token {
                Token::Let => Declare::Let,
                Token::Const => {
                    return Err(SyntaxError::DeprecatedError(DeprecatedError::const_statement(
                        &declare.span,
                    )));
                    //Declare::Const
                }
                _ => unimplemented!(),
            },
            variable_names,
            type_,
            value: expr,
        })
    }
}
