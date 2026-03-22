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

mod build;
mod new;

use crate::errors::Result;
use clap::builder::Styles;
use clap::builder::styling::{AnsiColor, Style};
use clap::{Args, FromArgMatches, Parser, Subcommand, crate_name};

#[derive(Parser)]
#[command(name = crate_name!(), version, about)]
pub struct Cli {
    #[arg(
        short,
        long,
        default_value = "package.toml",
        help = "Path to the package configuration file"
    )]
    config: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(about = "Initialize a new project")]
    New(new::CommandNew),

    #[command(about = "Build the project")]
    Build(build::CommandBuild),
}

fn get_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default().bold())
        .error(AnsiColor::Red.on_default().bold())
        .usage(AnsiColor::Green.on_default().bold())
        .literal(Style::new().bold())
        .placeholder(Style::new().italic())
        .valid(AnsiColor::Green.on_default())
        .invalid(AnsiColor::Red.on_default())
        .context(AnsiColor::Magenta.on_default())
}

pub fn parse(args: Vec<String>) -> Cli {
    let cli = clap::Command::new(crate_name!())
        .styles(get_styles())
        .help_expected(true);
    let cli = Cli::augment_args(cli);
    let matches = cli.get_matches_from(args);

    Cli::from_arg_matches(&matches)
        .map_err(|err| err.exit())
        .unwrap()
}

pub fn run(cli: Cli) -> Result<()> {
    match &cli.command {
        Command::New(n) => new::run(&cli, n, &vfs::PhysicalFS::new("/").into()),
        Command::Build(b) => build::run(&cli, b),
    }
}

#[cfg(test)]
mod test {
    use crate::cli::{Command, parse};
    use pretty_assertions::assert_eq;

    fn make_args(args: Vec<&str>) -> Vec<String> {
        args.iter().map(|&arg| arg.parse().unwrap()).collect()
    }

    #[test]
    fn it_parses_command_new_args() {
        let result = parse(make_args(vec!["fil", "new", "--name", "foo"]));
        match result.command {
            Command::New(n) => assert_eq!("foo", n.name.unwrap()),
            Command::Build(_) => panic!("Should have parsed command new"),
        }
    }

    #[test]
    fn it_parses_command_new_args_default() {
        let result = parse(make_args(vec!["fil", "new"]));
        match result.command {
            Command::New(n) => assert_eq!(None, n.name),
            Command::Build(_) => panic!("Should have parsed command new"),
        }
    }

    #[test]
    fn it_parses_command_build_args() {
        let result = parse(make_args(vec!["fil", "build", "-o", "dist"]));
        match result.command {
            Command::New(_) => panic!("Should have parsed command build"),
            Command::Build(b) => assert_eq!("dist", b.out_dir.unwrap()),
        }
    }

    #[test]
    fn it_parses_command_build_args_default() {
        let result = parse(make_args(vec!["fil", "build"]));
        match result.command {
            Command::New(_) => panic!("Should have parsed command build"),
            Command::Build(b) => assert_eq!("build", b.out_dir.unwrap()),
        }
    }
}
