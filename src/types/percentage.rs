//! This file implements percentage types.

use std::{error, fmt::Display};

/// This error is raised when a `UPercentage100` is
/// attempted to be constructed from a value that is not
/// a whole percentage between 0 and 100 inclusive.
#[derive(Debug, Clone)]
pub struct PercentageOutOfRange;

impl Display for PercentageOutOfRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Percentage value is out of range.")
    }
}

impl error::Error for PercentageOutOfRange {}

/// The Unsigned percentage up to 100 type represents a
/// verified percentage as a u32 that is no more than 100.
/// It implements `TryFrom<u32>`, and a `u32` can be
/// retrieved using `From<UPercent100>`.
#[derive(Debug, Clone)]
pub struct UPercent100 {
    value: u32,
}

impl TryFrom<u32> for UPercent100 {
    type Error = PercentageOutOfRange;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match (0_u32..=100_u32).contains(&value) {
            false => Err(PercentageOutOfRange),
            true => Ok(Self { value }),
        }
    }
}

impl From<UPercent100> for u32 {
    fn from(value: UPercent100) -> Self {
        value.value
    }
}
