//! This module contains the `Uf32` type, a wrapper around an `f32` that
//! assures that the value is positive. `Uf32` is immutable, and can only
//! be constructed or destructed for `f32`.

use std::{error, fmt::Display};

#[derive(Debug, Clone)]
pub struct NegativeFloatError;

impl Display for NegativeFloatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot construct a Uf32 from a negative value.")
    }
}

impl error::Error for NegativeFloatError {}

/// `Uf32` is a wrapper type around an `f32`, such that the `f32` is known to be
/// greater than or equal to `0_f32`. `Uf32` is always immutable.
///
/// # Example
/// Basic usage:
/// ```
/// use uf32::Uf32;
///
/// let a: Uf32 = Uf32::try_from(-394.34_f32).unwrap(); // Raises an error
/// let b: Uf32 = Uf32::try_from(29.5678_f32).unwrap();
/// assert!(29.5678_f32 == b.into());
/// ```
///
/// Implements `TryFrom<f32> for Uf32` and `Into<Uf32> for f32`. The inner `f32` value
/// is private so that the required checks can be done when constructing and so the value
/// can not be mutated at runtime for `mut Uf32`. Hence, it must either be
/// constructed or destructed via the methods provided.
#[derive(Debug, Clone, Copy)]
pub struct Uf32 {
    value: f32,
}

impl TryFrom<f32> for Uf32 {
    type Error = NegativeFloatError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value < 0.0 {
            Err(NegativeFloatError)
        } else {
            Ok(Self { value })
        }
    }
}

impl From<Uf32> for f32 {
    fn from(value: Uf32) -> Self {
        value.value
    }
}
