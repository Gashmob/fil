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
#[cfg(test)]
mod test;

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
    command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(about = "Initialize a new package")]
    New(new::CommandNew),

    #[command(about = "Build the package")]
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
