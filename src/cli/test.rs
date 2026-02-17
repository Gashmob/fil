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

mod cli {
    use crate::cli::{Command, parse};
    use pretty_assertions::assert_eq;

    fn make_args(args: Vec<&str>) -> Vec<String> {
        args.iter().map(|&arg| arg.parse().unwrap()).collect()
    }

    #[test]
    fn it_parses_config_arg() {
        let result = parse(make_args(vec!["fil", "--config", "hello.toml"]));
        assert_eq!("hello.toml", result.config);
    }

    #[test]
    fn it_parses_command_new_args() {
        let result = parse(make_args(vec!["fil", "new", "--name", "foo"]));
        if let Some(command) = result.command {
            match command {
                Command::New(n) => assert_eq!("foo", n.name.unwrap()),
                Command::Build(_) => panic!("Should have parsed command new"),
            }
        } else {
            panic!("Should have parsed command new");
        }
    }

    #[test]
    fn it_parses_command_new_args_default() {
        let result = parse(make_args(vec!["fil", "new"]));
        if let Some(command) = result.command {
            match command {
                Command::New(n) => assert_eq!(None, n.name),
                Command::Build(_) => panic!("Should have parsed command new"),
            }
        } else {
            panic!("Should have parsed command new");
        }
    }

    #[test]
    fn it_parses_command_build_args() {
        let result = parse(make_args(vec!["fil", "build", "-o", "dist"]));
        if let Some(command) = result.command {
            match command {
                Command::New(_) => panic!("Should have parsed command build"),
                Command::Build(b) => assert_eq!("dist", b.out_dir.unwrap()),
            }
        } else {
            panic!("Should have parsed command build");
        }
    }

    #[test]
    fn it_parses_command_build_args_default() {
        let result = parse(make_args(vec!["fil", "build"]));
        if let Some(command) = result.command {
            match command {
                Command::New(_) => panic!("Should have parsed command build"),
                Command::Build(b) => assert_eq!("build", b.out_dir.unwrap()),
            }
        } else {
            panic!("Should have parsed command build");
        }
    }
}

mod new {
    use crate::cli::new::CommandNew;
    use crate::cli::{Cli, new};
    use pretty_assertions::assert_eq;

    #[test]
    fn it_returns_ok() {
        assert_eq!(
            true,
            new::run(
                &Cli {
                    config: "".to_string(),
                    command: None
                },
                &CommandNew { name: None }
            )
            .is_ok()
        );
    }
}

mod build {
    use crate::cli::build::CommandBuild;
    use crate::cli::{Cli, build};
    use pretty_assertions::assert_eq;

    #[test]
    fn it_returns_ok() {
        assert_eq!(
            true,
            build::run(
                &Cli {
                    config: "".to_string(),
                    command: None
                },
                &CommandBuild { out_dir: None }
            )
            .is_ok()
        );
    }
}
