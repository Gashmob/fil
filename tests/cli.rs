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

mod common;

#[test]
fn exec_fil_help() {
    let output = common::exec_fil(vec!["help"]);
    pretty_assertions::assert_eq!(
        "Main tool for fil language

Usage: fil [OPTIONS] [COMMAND]

Commands:
  new    Initialize a new package
  build  Build the package
  help   Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>  Path to the package configuration file [default: package.toml]
  -h, --help             Print help
  -V, --version          Print version
",
        output.stdout
    );
    pretty_assertions::assert_eq!(true, output.status.success());
}

#[test]
fn exec_fil_version() {
    let output = common::exec_fil(vec!["--version"]);
    pretty_assertions::assert_eq!(
        "fil 0.1.0
",
        output.stdout
    );
    pretty_assertions::assert_eq!(true, output.status.success());
}

#[test]
fn exec_fil_help_new() {
    let output = common::exec_fil(vec!["help", "new"]);
    pretty_assertions::assert_eq!(
        "Initialize a new package

Usage: fil new [OPTIONS]

Options:
  -n, --name <NAME>
          Name of the package. If '.' is given, will initialize the package in current directory and
          use its name for package name

  -h, --help
          Print help (see a summary with '-h')
",
        output.stdout
    );
    pretty_assertions::assert_eq!(true, output.status.success());
}

#[test]
fn exec_fil_help_build() {
    let output = common::exec_fil(vec!["help", "build"]);
    pretty_assertions::assert_eq!(
        "Build the package

Usage: fil build [OPTIONS]

Options:
  -o, --out-dir <OUT_DIR>  Build destination directory [default: build]
  -h, --help               Print help
",
        output.stdout
    );
    pretty_assertions::assert_eq!(true, output.status.success());
}
