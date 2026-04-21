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

use inkwell::builder::BuilderError;

pub fn format_builder_error(error: &BuilderError) -> String {
    format!("{error}")
}

#[cfg(test)]
mod test {
    use crate::build::ir::builder_error_formatter::format_builder_error;
    use inkwell::builder::{BuilderError, CmpxchgOrderingError};
    use inkwell::error::AlignmentError;
    use inkwell::values::AtomicError;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_it_returns_error_message() {
        assert_str_eq!(
            "Builder position is not set",
            format_builder_error(&BuilderError::UnsetPosition)
        );
        assert_str_eq!(
            "Alignment error",
            format_builder_error(&BuilderError::AlignmentError(AlignmentError::Unsized))
        );
        assert_str_eq!(
            "Aggregate extract index out of range",
            format_builder_error(&BuilderError::ExtractOutOfRange)
        );
        assert_str_eq!(
            "The bitwidth of value must be a power of 2 and greater than or equal to 8.",
            format_builder_error(&BuilderError::BitwidthError)
        );
        assert_str_eq!(
            "Pointee type does not match the value's type",
            format_builder_error(&BuilderError::PointeeTypeMismatch)
        );
        assert_str_eq!(
            "Values must have the same type",
            format_builder_error(&BuilderError::NotSameType)
        );
        assert_str_eq!(
            "Values must have pointer or integer type",
            format_builder_error(&BuilderError::NotPointerOrInteger)
        );
        assert_str_eq!(
            "Cmpxchg ordering error or mismatch",
            format_builder_error(&BuilderError::CmpxchgOrdering(
                CmpxchgOrderingError::WeakerThanMonotic
            ))
        );
        assert_str_eq!(
            "Atomic ordering error",
            format_builder_error(&BuilderError::AtomicOrdering(AtomicError::ReleaseOnLoad))
        );
        assert_str_eq!(
            "GEP pointee is not a struct",
            format_builder_error(&BuilderError::GEPPointee)
        );
        assert_str_eq!(
            "GEP index out of range",
            format_builder_error(&BuilderError::GEPIndex)
        );
    }
}
