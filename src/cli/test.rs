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

mod new {
    use crate::cli::new::CommandNew;
    use crate::cli::{Cli, Command, new};
    use pretty_assertions::assert_eq;
    use std::io::Read;
    use vfs::{MemoryFS, VfsPath};

    fn random_name() -> String {
        format!("project_{}", rand::random::<u64>())
    }

    fn run_new(path: &VfsPath) {
        let result = new::run(
            &Cli {
                config: "".to_string(),
                command: Command::New(CommandNew {
                    name: Some(path.as_str().to_string()),
                    git: Some(false),
                }),
            },
            &CommandNew {
                name: Some(path.as_str().to_string()),
                git: Some(false),
            },
            &path,
        );
        assert_eq!(true, result.is_ok());
    }

    #[test]
    fn it_creates_project_dir() {
        let root = VfsPath::new(MemoryFS::new());
        let name = random_name();
        let path = root.join(format!("/tmp/{}", name)).unwrap();
        run_new(&path);

        assert_eq!(true, path.is_dir().unwrap());
        let content: Vec<_> = path.read_dir().unwrap().collect();
        assert_eq!(vec![path.join("package.toml").unwrap()], content);
        let mut package_content = String::new();
        println!("{:?}", root);
        path.join("package.toml")
            .unwrap()
            .open_file()
            .unwrap()
            .read_to_string(&mut package_content)
            .unwrap();
        assert_eq!(
            format!(
                "[package]
name = {}",
                name
            ),
            package_content
        );
    }
}

mod build {
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
