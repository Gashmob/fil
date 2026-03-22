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

use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
pub struct GenericError {
    message: String,
}

impl GenericError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for GenericError {}

#[derive(Debug, Clone)]
pub struct NotImplementedError {
    feature_name: String,
}

impl NotImplementedError {
    pub fn new(feature_name: &str) -> Self {
        Self {
            feature_name: feature_name.to_string(),
        }
    }
}

impl fmt::Display for NotImplementedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is not yet implemented", self.feature_name)
    }
}

impl error::Error for NotImplementedError {}

#[cfg(test)]
mod test {
    use crate::errors::{GenericError, NotImplementedError};
    use pretty_assertions::assert_eq;

    #[test]
    fn it_stores_message() {
        assert_eq!(
            "My error message",
            GenericError::new("My error message").to_string()
        );
    }

    #[test]
    fn it_tells_feature_is_not_implemented() {
        assert_eq!(
            "foo is not yet implemented",
            NotImplementedError::new("foo").to_string()
        );
    }
}
