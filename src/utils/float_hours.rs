//! This file contains a type for a number of hours as a
//! float, that is between 0 and 24.

use std::{error, fmt::Display};

/// This Error represents that the hours provided to `DayHours`
/// is not in the 24 hour range. Implements `std::error::Error`.
#[derive(Debug, Clone)]
pub struct HoursNotInDay;

impl Display for HoursNotInDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid: hours is not between 0 and 24.")
    }
}

impl error::Error for HoursNotInDay {}

/// This is a type that contains an `f32` that is verified to
/// be in the range 0 to 24 inclusive. It can be constructed
/// using `try_from(value: f32)`, and can be destructed to
/// an `f32` with `from(value: DayHours)`. `
pub struct DayHours {
    hours: f32,
}

impl TryFrom<f32> for DayHours {
    type Error = HoursNotInDay;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        match (0_f32..=24_f32).contains(&value) {
            false => Err(HoursNotInDay),
            true => Ok(Self { hours: value }),
        }
    }
}

impl From<DayHours> for f32 {
    fn from(value: DayHours) -> Self {
        value.hours
    }
}