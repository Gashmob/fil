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

use crate::fault;
use crate::fault::Fault;
use std::process;
use vfs::VfsPath;

pub fn create_project(name: &String, git: &bool, filesystem: &VfsPath) -> fault::Result<()> {
    let name = sanitize_name(name);
    get_path(&filesystem, &name)
        .map(|path| (path.clone(), get_name(&name, &path)))
        .and_then(|(path, name)| check_path_is_empty(&path).map(|_| (path, name)))
        .and_then(|(path, name)| {
            path.create_dir_all()
                .map_err(|error| Fault::from_error(Box::from(error)))
                .map(|_| (path, name))
        })
        .and_then(|(path, name)| init_project(&path, &name).map(|_| path))
        .and_then(|path| init_git(&git, &path))
}

fn get_path(filesystem: &VfsPath, name: &String) -> fault::Result<VfsPath> {
    if name.starts_with("/") {
        filesystem
            .root()
            .join(&name)
            .map_err(|error| Fault::from_error(Box::from(error)))
    } else {
        std::env::current_dir()
            .map_err(|error| Fault::from_error(Box::from(error)))
            .and_then(|current_dir_path| {
                if let Some(current_dir) = current_dir_path.to_str() {
                    Ok(String::from(current_dir))
                } else {
                    Err(Fault::from_message(""))
                }
            })
            .and_then(|current_dir| {
                filesystem
                    .join(current_dir)
                    .map_err(|error| Fault::from_error(Box::from(error)))
            })
            .and_then(|_| {
                filesystem
                    .join(&name)
                    .map_err(|error| Fault::from_error(Box::from(error)))
            })
    }
}

fn get_name(name: &String, path: &VfsPath) -> String {
    if name.contains("/") {
        path.filename()
    } else {
        name.clone()
    }
}

fn check_path_is_empty(path: &VfsPath) -> fault::Result<()> {
    path.exists()
        .map_err(|error| Fault::from_error(Box::from(error)))
        .and_then(|exists| {
            if exists {
                path.read_dir()
                    .map_err(|error| Fault::from_error(Box::from(error)))
                    .and_then(|read_dir| {
                        if read_dir.count() > 0 {
                            Err(Fault::from_message(
                                format!("Directory {} is not empty", path.as_str()).as_str(),
                            ))
                        } else {
                            Ok(())
                        }
                    })
            } else {
                Ok(())
            }
        })
}

fn init_project(path: &VfsPath, name: &String) -> fault::Result<()> {
    path.join("package.toml")
        .map_err(|error| Fault::from_error(Box::from(error)))
        .and_then(|file| {
            file.create_file()
                .map_err(|error| Fault::from_error(Box::from(error)))
        })
        .and_then(|mut file_stream| {
            write!(file_stream, "[package]\nname = {}", name)
                .map_err(|error| Fault::from_error(Box::from(error)))
        })
}

fn init_git(git: &bool, path: &VfsPath) -> fault::Result<()> {
    if *git {
        process::Command::new("git")
            .args(vec!["init", path.as_str()])
            .output()
            .map_err(|error| Fault::from_error(Box::from(error)))
            .and(Ok(()))
    } else {
        Ok(())
    }
}

fn sanitize_name(name: &String) -> String {
    let parts: Vec<_> = name.trim().split_whitespace().collect();
    parts.join("-").replace("*", "-")
}

#[cfg(test)]
mod test {
    use crate::new::{check_path_is_empty, sanitize_name};
    use pretty_assertions::{assert_eq, assert_str_eq};
    use vfs::{MemoryFS, VfsPath};

    #[test]
    fn test_check_path() {
        let root = VfsPath::new(MemoryFS::new());
        root.join("foo").unwrap().create_dir().unwrap();
        root.join("bar").unwrap().create_dir().unwrap();
        root.join("bar/lorem").unwrap().create_file().unwrap();

        assert_eq!(
            true,
            check_path_is_empty(&root.join("foo").unwrap()).is_ok()
        );
        assert_eq!(
            true,
            check_path_is_empty(&root.join("bar").unwrap()).is_err()
        );
    }

    #[test]
    fn test_sanitize_name() {
        assert_str_eq!("foo", sanitize_name(&"foo".to_string()));
        assert_str_eq!("foo-bar", sanitize_name(&"foo bar".to_string()));
        assert_str_eq!("foo-bar", sanitize_name(&"foo-bar".to_string()));
        assert_str_eq!("foo_bar", sanitize_name(&"foo_bar".to_string()));
        assert_str_eq!("foo", sanitize_name(&"   foo   ".to_string()));
        assert_str_eq!("foo-bar", sanitize_name(&"   foo   bar   ".to_string()));
        assert_str_eq!("foo&bar", sanitize_name(&"foo&bar".to_string()));
        assert_str_eq!("foo-bar", sanitize_name(&"foo*bar".to_string()));
    }
}
