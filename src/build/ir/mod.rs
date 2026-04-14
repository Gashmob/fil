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

mod builder_error_formatter;

use crate::build::grammar::ast::{Expr, Opcode};
use crate::build::ir::builder_error_formatter::format_builder_error;
use crate::fault;
use crate::fault::Fault;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, IntValue};

struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    fn compile_expr(&mut self, expr: &Expr) -> fault::Result<IntValue<'ctx>> {
        match expr {
            Expr::Number(n) => Ok(self.context.i32_type().const_int(*n as u64, false)),
            Expr::Op(l, o, r) => self.compile_operator(l, o, r),
        }
    }

    fn compile_operator(
        &mut self,
        left: &Expr,
        operator: &Opcode,
        right: &Expr,
    ) -> fault::Result<IntValue<'ctx>> {
        let lhs = self.compile_expr(left)?;
        let rhs = self.compile_expr(right)?;

        match operator {
            Opcode::Mul => self.builder.build_int_mul(lhs, rhs, "fil_mul"),
            Opcode::Div => self.builder.build_int_unsigned_div(lhs, rhs, "fil_dib"),
            Opcode::Add => self.builder.build_int_add(lhs, rhs, "fil_add"),
            Opcode::Sub => self.builder.build_int_sub(lhs, rhs, "fil_sub"),
        }
        .map_err(|err| Fault::from_message(format_builder_error(&err).as_str()))
    }

    fn entry_function(&self) -> fault::Result<FunctionValue<'ctx>> {
        let function_type = self.context.i32_type().fn_type(&[], false);
        let function_value = self.module.add_function("main", function_type, None);

        Ok(function_value)
    }

    pub fn compile(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        expr: &Expr,
    ) -> fault::Result<FunctionValue<'ctx>> {
        let mut compiler = Self {
            context,
            builder,
            module,
        };

        let function = compiler.entry_function()?;
        let entry = compiler.context.append_basic_block(function, "entry");
        compiler.builder.position_at_end(entry);

        let body = compiler.compile_expr(expr)?;

        compiler
            .builder
            .build_return(Some(&body))
            .map_err(|err| Fault::from_error(Box::from(err)))?;

        if function.verify(true) {
            Ok(function)
        } else {
            unsafe {
                function.delete();
            }
            Err(Fault::from_message("Invalid generated main function"))
        }
    }
}

pub fn transform_to_ir(expr: &Expr) -> fault::Result<String> {
    let context = Context::create();
    let builder = context.create_builder();
    let module = context.create_module("fil");

    Compiler::compile(&context, &builder, &module, expr).map(|ir| format!("{ir}"))
}

#[cfg(test)]
mod test {
    use crate::build::grammar::grammar;
    use crate::build::ir::transform_to_ir;
    use crate::fault;
    use crate::fault::Fault;
    use pretty_assertions::assert_str_eq;

    fn generate_ir(input: &str) -> fault::Result<String> {
        let expr = grammar::ExprParser::new()
            .parse(input)
            .map_err(|_| Fault::from_message("Failed to parse input"))?;
        transform_to_ir(&expr)
    }

    #[test]
    fn test_it_generates_some_ir() {
        assert_str_eq!(
            "\"define i32 @main() {\\nentry:\\n  ret i32 2\\n}\\n\"",
            generate_ir("1+1").unwrap()
        );
    }
}
