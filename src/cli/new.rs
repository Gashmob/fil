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
use crate::fault;
use crate::fault::Fault;
use crate::new::create_project;
use clap::Args;
use cliclack::ProgressBar;

#[derive(Args)]
pub struct CommandNew {
    #[arg(
        short,
        long,
        help = "Name of the project",
        long_help = "Name of the project. Can be be a path, in this case the basename of it will be used for the project name."
    )]
    pub name: Option<String>,

    #[arg(short, long, help = "Call git init in the project?")]
    pub git: Option<bool>,
}

pub fn run(_cli: &Cli, command: &CommandNew, filesystem: &vfs::path::VfsPath) -> fault::Result<()> {
    cliclack::intro(console::style(" New project ").on_green().black())
        .and_then(|_| cliclack::log::success("Let's create an awesome project 🤘"))
        .map_err(|error| Fault::from_error(Box::from(error)))
        .map(|_| {
            let spinner = cliclack::spinner();
            spinner.start("Initializing the project");
            spinner
        })
        .and_then(|spinner| get_name(&command).map(|name| (spinner, name)))
        .and_then(|(spinner, name)| get_git(&command).map(|git| (spinner, name, git)))
        .and_then(|(spinner, name, git)| {
            create_project(&name, &git, &filesystem).map(|_| (spinner, name))
        })
        .and_then(|(spinner, name): (ProgressBar, String)| {
            spinner.stop("Done!");

            cliclack::note(
                "Project created! 🚀 ",
                format!(
                    "{}\n{}{}\n",
                    console::style("Next steps:").bold(),
                    if name == "." {
                        String::new()
                    } else {
                        console::style(format!("cd {name}\n")).dim().to_string()
                    },
                    "Enjoy!"
                ),
            )
            .map_err(|error| Fault::from_error(Box::from(error)))
        })
        .and_then(|_| {
            cliclack::outro(format!(
                "Got problems? {}",
                console::style("https://github.com/Gashmob/fil/issues/new/choose")
                    .yellow()
                    .underlined()
            ))
            .map_err(|error| Fault::from_error(Box::from(error)))
        })
}

fn get_name(command: &CommandNew) -> fault::Result<String> {
    if let Some(given_name) = &command.name {
        cliclack::log::step(format!(
            "Project will be created with name: {}",
            console::style(given_name).bold()
        ))
        .map(|_| given_name.clone())
    } else {
        cliclack::input("How do you want to call it?")
            .placeholder("blazing-fast-forward")
            .validate(|input: &String| {
                if input.is_empty() {
                    Err("Please enter a valid name")
                } else {
                    Ok(())
                }
            })
            .interact()
    }
    .map_err(|error| Fault::from_error(Box::from(error)))
}

fn get_git(command: &CommandNew) -> fault::Result<bool> {
    if let Some(given_git) = command.git {
        if given_git {
            cliclack::log::step(format!(
                "{} will be called",
                console::style("git init").bold()
            ))
        } else {
            Ok(())
        }
        .map(|_| given_git.clone())
    } else {
        cliclack::confirm("Do you want to init git?").interact()
    }
    .map_err(|error| Fault::from_error(Box::from(error)))
}

#[cfg(test)]
mod test {
    use crate::cli::new::CommandNew;
    use crate::cli::{Cli, Command, new};
    use pretty_assertions::{assert_eq, assert_str_eq};
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
        path.join("package.toml")
            .unwrap()
            .open_file()
            .unwrap()
            .read_to_string(&mut package_content)
            .unwrap();
        assert_str_eq!(
            format!(
                "[package]
name = {}",
                name
            ),
            package_content
        );
    }
}
