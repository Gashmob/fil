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

use std::process::{Command, ExitStatus};

pub struct FormattedOutput {
    pub status: ExitStatus,
    pub stdout: String,
    pub stderr: String,
}

pub fn exec_fil(args: Vec<&str>) -> FormattedOutput {
    let output = Command::new("./target/debug/fil")
        .args(args)
        .output()
        .expect("Failed to execute process");

    let mut stdout = String::new();
    stdout.push_str(match str::from_utf8(&output.stdout) {
        Ok(val) => val,
        Err(_) => panic!("Got non utf-8 data from stdout"),
    });
    let mut stderr = String::new();
    stderr.push_str(match str::from_utf8(&output.stderr) {
        Ok(val) => val,
        Err(_) => panic!("Got non utf-8 data from stderr"),
    });

    FormattedOutput {
        status: output.status,
        stdout,
        stderr,
    }
}
