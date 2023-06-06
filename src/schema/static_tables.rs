//! This module represents the types for the values in
//! the static tables of the database, e.g. unit names to
//! for optimisation using foreign integer keys as integers
//! are smaller than VarChars. The types

pub trait Name {}

/// This enum represents the names of locations that are
/// in the LDS for the local data.
#[derive(Debug, Clone, Copy)]
pub enum LocalName {
    Camborne,
    Heathrow,
    Hurn,
    Leeming,
    Leuchars,
}
impl Name for LocalName {}

/// This enum represents the names of locations that are
/// in the LDS for overseas data.
#[derive(Debug, Clone, Copy)]
pub enum OverseasName {
    Beijing,
    Jacksonville,
    Perth,
}
impl Name for OverseasName {}

/// This enum outlines the Beaufort unit conversions. It is
/// discrete and empirical. **This enum only covers the
/// conversions for the _breezes_, as they are the only ones
/// present in the LDS.**
pub enum Beaufort {
    Calm,
    Light,
    Gentle,
    Moderate,
    Fresh,
    Strong,
}

/// This direction is composed of a tuple, containing at least
/// one primary cardinal direction, and then another two optionals.
/// This represents variants like N, SW, and ESE for example.
///
/// Note: **This does _NOT_ account for possible logically invalid
/// directions such as: EEE, NS, WEN, etc.** For now, for simplicity,
/// I am forgoing this possibility of invalid state as I think we can
/// safely assume that the directions provided in the LDS are valid.
pub type Cardinal3 = (Direction, Option<Direction>, Option<Direction>);

/// The cardinal direction enum, for NSEW.
pub enum Direction {
    N,
    S,
    E,
    W,
}

// TODO: add some function for safely constructing a Cardinal3
