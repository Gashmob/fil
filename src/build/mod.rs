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

mod ast;
mod grammar;
mod parse_error_formatter;
mod validator;

use crate::build::ast::Expr;
use crate::build::parse_error_formatter::format_parse_error;
use crate::build::validator::validate;
use crate::cli::Cli;
use crate::cli::build::CommandBuild;
use crate::fault;
use crate::fault::Fault;

pub fn build(
    _cli: &Cli,
    _command: &CommandBuild,
    filesystem: &vfs::path::VfsPath,
) -> fault::Result<()> {
    let expr = filesystem
        .join("src/main.fil")
        .map_err(|error| Fault::from_error(Box::from(error)))
        .and_then(|main_source_file| parse_file(&main_source_file))
        .and_then(|expr| validate(&expr));
    // TODO:
    //  - match visitor to build IR (llvm?)
    //  - linking into executable

    expr.and_then(|_| Err(Fault::from_message("Not yet implemented")))
}

fn parse_file(main_source_file: &vfs::path::VfsPath) -> fault::Result<Box<Expr>> {
    main_source_file
        .read_to_string()
        .map_err(|error| Fault::from_error(Box::from(error)))
        .and_then(|content| {
            grammar::ExprParser::new()
                .parse(content.as_str())
                .map_err(|error| Fault::from_message(format_parse_error(error, &content).as_str()))
        })
}

#[cfg(test)]
mod test {
    use crate::build::{grammar, parse_file};
    use pretty_assertions::{assert_eq, assert_str_eq};
    use vfs::{MemoryFS, VfsPath};

    #[test]
    fn test_grammar() {
        let expr = grammar::ExprParser::new().parse("22 * 44 + 66").unwrap();
        assert_str_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");
    }

    #[test]
    fn test_parse_file() {
        let root = VfsPath::new(MemoryFS::new());
        root.join("src").unwrap().create_dir().unwrap();
        let source_file = root.join("src/main.rs").unwrap();
        source_file.create_file().unwrap();
        source_file
            .append_file()
            .unwrap()
            .write_fmt(format_args!("1 + 3 * 12 -4"))
            .unwrap();

        let expr = parse_file(&source_file).unwrap();
        assert_str_eq!(&format!("{:?}", expr), "((1 + (3 * 12)) - 4)");
    }

    #[test]
    fn test_parse_file_err() {
        let root = VfsPath::new(MemoryFS::new());
        root.join("src").unwrap().create_dir().unwrap();
        let source_file = root.join("src/main.rs").unwrap();
        source_file.create_file().unwrap();
        source_file
            .append_file()
            .unwrap()
            .write_fmt(format_args!("1 + hello"))
            .unwrap();

        let result = parse_file(&source_file);
        assert_eq!(result.is_err(), true);
        assert_str_eq!(
            format!("{}", result.err().unwrap()),
            "Invalid token at line 1:

 1 | 1 + hello
         ^
"
        );
    }
}
