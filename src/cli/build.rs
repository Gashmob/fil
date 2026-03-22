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

use crate::cli::Cli;
use crate::errors::NotImplementedError;
use crate::errors::Result;
use clap::Args;

#[derive(Args)]
pub struct CommandBuild {
    #[arg(
        short,
        long,
        default_value = "build",
        help = "Build destination directory"
    )]
    pub out_dir: Option<String>,
}

pub fn run(_cli: &Cli, _command: &CommandBuild) -> Result<()> {
    Err(NotImplementedError::new("build command").into())
}

#[cfg(test)]
mod test {
    use crate::cli::build::CommandBuild;
    use crate::cli::{Cli, Command, build};
    use pretty_assertions::assert_eq;

    #[test]
    fn it_returns_err() {
        let result = build::run(
            &Cli {
                config: "".to_string(),
                command: Command::Build(CommandBuild { out_dir: None }),
            },
            &CommandBuild { out_dir: None },
        );
        assert_eq!(true, result.is_err());
        assert_eq!(
            "build command is not yet implemented",
            result.unwrap_err().to_string()
        );
    }
}
