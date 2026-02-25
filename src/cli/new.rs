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
use crate::errors::{GenericError, Result};
use clap::Args;
use std::process;

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

    let name = if let Some(given_name) = &command.name {
        cliclack::log::step(format!(
            "Project will be created with name: {}",
            console::style(given_name).bold()
        ))?;
        given_name
    } else {
        &cliclack::input("How do you want to call it?")
            .placeholder("blazing-fast-forward")
            .validate(|input: &String| {
                if input.is_empty() {
                    Err("Please enter a valid name")
                } else {
                    Ok(())
                }
            })
            .interact()?
    };

    let git = if let Some(given_git) = &command.git {
        if *given_git {
            cliclack::log::step(format!(
                "{} will be called",
                console::style("git init").bold()
            ))?;
        }
        given_git
    } else {
        &cliclack::confirm("Do you want to init git?").interact()?
    };

    let spinner = cliclack::spinner();
    spinner.start("Initializing the project");
    create_project(name, git, &filesystem).and_then(|_| {
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

fn create_project(name: &String, git: &bool, filesystem: &vfs::path::VfsPath) -> Result<()> {
    let name = sanitize_name(name);
    let path = if name.starts_with("/") {
        filesystem.root().join(&name)?
    } else {
        filesystem
            .join(std::env::current_dir()?.to_str().unwrap())?
            .join(&name)?
    };
    let name = if name.contains("/") {
        path.filename()
    } else {
        name.clone()
    };

    check_path(&path)
        .and_then(|_| {
            path.create_dir_all()
                .map_err(|err| GenericError::new(err.to_string().as_str()).into())
        })
        .and_then(|_| {
            path.join("package.toml")
                .map_err(|err| GenericError::new(err.to_string().as_str()).into())
                .and_then(|file| {
                    file.create_file()
                        .map_err(|err| GenericError::new(err.to_string().as_str()).into())
                })
                .and_then(|mut file_stream| {
                    write!(file_stream, "[package]\nname = {}", name)
                        .map_err(|err| GenericError::new(err.to_string().as_str()).into())
                })
        })
        .and_then(|_| {
            if *git {
                process::Command::new("git")
                    .args(vec!["init", path.as_str()])
                    .output()
                    .map_err(|err| GenericError::new(err.to_string().as_str()).into())
                    .and(Ok(()))
            } else {
                Ok(())
            }
        })
}

fn check_path(path: &vfs::path::VfsPath) -> Result<()> {
    path.exists()
        .map_err(|err| GenericError::new(err.to_string().as_str()).into())
        .and_then(|exists| {
            if exists {
                path.read_dir()
                    .map_err(|err| GenericError::new(err.to_string().as_str()).into())
                    .and_then(|read_dir| {
                        if read_dir.count() > 0 {
                            Err(GenericError::new(
                                format!("Directory {} is not empty", path.as_str()).as_str(),
                            )
                            .into())
                        } else {
                            Ok(())
                        }
                    })
            } else {
                Ok(())
            }
        })
}

fn sanitize_name(name: &String) -> String {
    let parts: Vec<_> = name.trim().split_whitespace().collect();
    parts.join("-").replace("*", "-")
}

#[cfg(test)]
mod test {
    use crate::cli::new::sanitize_name;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_sanitize_name() {
        assert_eq!("foo", sanitize_name(&"foo".to_string()));
        assert_eq!("foo-bar", sanitize_name(&"foo bar".to_string()));
        assert_eq!("foo-bar", sanitize_name(&"foo-bar".to_string()));
        assert_eq!("foo_bar", sanitize_name(&"foo_bar".to_string()));
        assert_eq!("foo", sanitize_name(&"   foo   ".to_string()));
        assert_eq!("foo-bar", sanitize_name(&"   foo   bar   ".to_string()));
        assert_eq!("foo&bar", sanitize_name(&"foo&bar".to_string()));
        assert_eq!("foo-bar", sanitize_name(&"foo*bar".to_string()));
    }
}
