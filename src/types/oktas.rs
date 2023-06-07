//! This is an Oktas type - represents cloud cover.

use std::{error, fmt::Display};

/// This error is raised when an `Oktas` is attempted
/// to be constructed using a value that is not in
/// the range 0-8 inclusive. Implements `std::error::Error`.
#[derive(Debug, Clone)]
pub struct NotAnOkta;

impl Display for NotAnOkta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Value is not an Okta.")
    }
}

impl error::Error for NotAnOkta {}

/// This type contains a u32 value that is in the range 0-8
/// inclusive to represent the Oktas unit for cloud coverage.
#[derive(Debug, Clone)]
pub struct Oktas {
    // Not stored as a u8 for example because it would
    // be too annoying to interface with other things.
    value: u32,
}

impl TryFrom<u32> for Oktas {
    type Error = NotAnOkta;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match (0_u32..=8_u32).contains(&value) {
            false => Err(NotAnOkta),
            true => Ok(Self { value }),
        }
    }
}

impl From<Oktas> for u32 {
    fn from(value: Oktas) -> Self {
        value.value
    }
}
