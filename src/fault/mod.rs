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

use std::fmt::Formatter;
use std::{error, fmt};

#[derive(Debug)]
pub struct Fault {
    message: Option<String>,
    error: Option<Box<dyn error::Error>>,
}

impl Fault {
    pub fn from_message(message: &str) -> Self {
        Self {
            message: Some(message.to_string()),
            error: None,
        }
    }

    pub fn from_error(error: Box<dyn error::Error>) -> Self {
        Self {
            message: None,
            error: Some(error),
        }
    }

    pub fn from_error_with_message(error: Box<dyn error::Error>, message: &str) -> Self {
        Self {
            message: Some(message.to_string()),
            error: Some(error),
        }
    }
}

impl fmt::Display for Fault {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match (&self.message, &self.error) {
            (Some(message), None) => write!(f, "{message}"),
            (Some(message), Some(error)) => write!(f, "{message}: {error}"),
            (None, Some(error)) => write!(f, "{error}"),
            _ => write!(f, "Got an unknown fault, please open an issue"),
        }
    }
}

pub type Result<T> = std::result::Result<T, Fault>;

#[cfg(test)]
mod test {
    use crate::fault::Fault;
    use pretty_assertions::assert_eq;
    use std::fmt::Formatter;
    use std::{error, fmt};

    #[derive(Debug)]
    struct ErrorStub {}

    impl fmt::Display for ErrorStub {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "Some stub error")
        }
    }

    impl error::Error for ErrorStub {}

    #[test]
    fn test_fault_from_message() {
        assert_eq!("Oh snap!", Fault::from_message("Oh snap!").to_string())
    }

    #[test]
    fn test_fault_from_error() {
        assert_eq!(
            "Some stub error",
            Fault::from_error(Box::new(ErrorStub {})).to_string()
        )
    }

    #[test]
    fn test_fault_from_error_with_message() {
        assert_eq!(
            "Oopsie: Some stub error",
            Fault::from_error_with_message(Box::new(ErrorStub {}), "Oopsie").to_string()
        )
    }
}
