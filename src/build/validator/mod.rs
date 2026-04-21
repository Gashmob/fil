// fil
// Copyright (C) 2026 - Present  fil contributors
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

use crate::build::grammar::ast::{Expr, Opcode};
use crate::fault;
use crate::fault::Fault;

pub fn validate(expr: &Expr) -> fault::Result<()> {
    validate_expression(expr).map(|_| ())
}

fn validate_expression(expr: &Expr) -> fault::Result<u32> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::Op(l, o, r) => validate_operator(l, o, r),
    }
}

fn validate_operator(left: &Expr, operator: &Opcode, right: &Expr) -> fault::Result<u32> {
    match operator {
        Opcode::Mul => validate_expression(right).and_then(|right_result| {
            validate_expression(left).map(|left_result| left_result * right_result)
        }),
        Opcode::Div => validate_expression(right).and_then(|right_result| {
            if right_result == 0 {
                Err(Fault::from_message("You cannot divide by 0"))
            } else {
                validate_expression(left).map(|left_result| left_result / right_result)
            }
        }),
        Opcode::Add => validate_expression(right).and_then(|right_result| {
            validate_expression(left).map(|left_result| left_result + right_result)
        }),
        Opcode::Sub => validate_expression(right).and_then(|right_result| {
            validate_expression(left).map(|left_result| left_result - right_result)
        }),
    }
}

#[cfg(test)]
mod test {
    use crate::build::grammar::grammar;
    use crate::build::validator::validate_expression;
    use crate::fault;
    use crate::fault::Fault;
    use pretty_assertions::{assert_eq, assert_str_eq};

    fn parse_and_validate(input: &str) -> fault::Result<u32> {
        grammar::ExprParser::new()
            .parse(input)
            .map_err(|_| Fault::from_message("Failed to parse expression"))
            .and_then(|expr| validate_expression(&expr))
    }

    #[test]
    fn test_validate_expression() {
        assert_eq!(12, parse_and_validate("12").unwrap());
        assert_eq!(5, parse_and_validate("2 + 3").unwrap());
        assert_eq!(1, parse_and_validate("3 - 2").unwrap());
        assert_eq!(6, parse_and_validate("2 * 3").unwrap());
        assert_eq!(2, parse_and_validate("4 / 2").unwrap());
        assert_eq!(1034, parse_and_validate("22 * 44 + 66").unwrap());
        assert_eq!(2420, parse_and_validate("22 * (44 + 66)").unwrap());

        assert_str_eq!(
            "You cannot divide by 0",
            format!("{}", parse_and_validate("4 / 0").err().unwrap())
        );
    }
}
