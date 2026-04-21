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

mod grammar;
mod ir;
mod validator;

use crate::build::grammar::parse_file;
use crate::build::ir::transform_to_ir;
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
        .and_then(|expr| validate(&expr).map(|_| expr))
        .and_then(|expr| transform_to_ir(&expr))
        .map(|ir| println!("{ir}"));
    // TODO:
    //  - linking into executable

    expr.and_then(|_| Err(Fault::from_message("Not yet implemented")))
}
