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
use crate::errors::Result;
use crate::new::create_project;
use clap::Args;

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

pub fn run(_cli: &Cli, command: &CommandNew, filesystem: &vfs::path::VfsPath) -> Result<()> {
    cliclack::intro(console::style(" New project ").on_green().black())?;

    cliclack::log::success("Let's create an awesome project 🤘")?;

    let name = get_name(&command)?;
    let git = get_git(&command)?;

    let spinner = cliclack::spinner();
    spinner.start("Initializing the project");
    create_project(&name, &git, &filesystem).and_then(|_| {
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
        )?;

        cliclack::outro(format!(
            "Got problems? {}",
            console::style("https://github.com/Gashmob/fil/issues/new/choose")
                .yellow()
                .underlined()
        ))?;
        Ok(())
    })
}

fn get_name(command: &&CommandNew) -> Result<String> {
    if let Some(given_name) = &command.name {
        cliclack::log::step(format!(
            "Project will be created with name: {}",
            console::style(given_name).bold()
        ))?;
        Ok(given_name.clone())
    } else {
        Ok(cliclack::input("How do you want to call it?")
            .placeholder("blazing-fast-forward")
            .validate(|input: &String| {
                if input.is_empty() {
                    Err("Please enter a valid name")
                } else {
                    Ok(())
                }
            })
            .interact()?)
    }
}

fn get_git(command: &CommandNew) -> Result<bool> {
    if let Some(given_git) = command.git {
        if given_git {
            cliclack::log::step(format!(
                "{} will be called",
                console::style("git init").bold()
            ))?;
        }
        Ok(given_git.clone())
    } else {
        Ok(cliclack::confirm("Do you want to init git?").interact()?)
    }
}

#[cfg(test)]
mod test {
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
