//! This file contains a type for a number of hours as a
//! float, that is between 0 and 24.

use std::{error::Error, fmt::Display};

/// This Error represents that the hours provided to `DayHours`
/// is not in the 24 hour range. Implements `std::error::Error`.
#[derive(Debug, Clone)]
pub struct HoursNotInDay;

impl Display for HoursNotInDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid: hours is not between 0 and 24.")
    }
}

impl Error for HoursNotInDay {}

/// This is a type that contains an `f32` that is verified to
/// be in the range 0 to 24 inclusive.
pub struct DayHours {
    hours: f32,
}

impl DayHours {
    /// Construct a new value, verifying the value is in the
    /// correct range for a 24 hour day.
    pub fn new(hours: f32) -> Result<Self, Box<dyn Error>> {
        match (0_f32..=24_f32).contains(&hours) {
            false => Err(Box::new(HoursNotInDay)),
            true => Ok(Self { hours: hours }),
        }
    }
}

impl Into<f32> for DayHours {
    fn into(self) -> f32 {
        self.hours
    }
}
