//! This implements a type to wrap different amounts of
//! degrees (deg).

use std::{error, fmt::Display};

/// This error is thrown when an UDegree360 is attempted
/// to be constructed using an invalid value. Implements
/// `std::error::Error`.
#[derive(Debug, Clone)]
pub struct NotADegree;

impl Display for NotADegree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Value is not a valid degrees value.")
    }
}

impl error::Error for NotADegree {}

/// This is an unsigned, whole degrees value that can
/// go up to and including 360 degrees. Can be constructed
/// using `TryFrom<u32>` and destructed to a `u32` with
/// `From<UDegree360>`.
#[derive(Debug, Clone)]
pub struct UDegree360 {
    value: u32,
}

impl TryFrom<u32> for UDegree360 {
    type Error = NotADegree;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match (0_u32..=360_u32).contains(&value) {
            false => Err(NotADegree),
            true => Ok(UDegree360 { value }),
        }
    }
}

impl From<UDegree360> for u32 {
    fn from(value: UDegree360) -> Self {
        value.value
    }
}
