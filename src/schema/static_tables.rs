//! This module represents the types for the values in
//! the static tables of the database, e.g. unit names to
//! for optimisation using foreign integer keys as integers
//! are smaller than VarChars. The types

use std::{error::Error, fmt::Display};

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
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    N,
    S,
    E,
    W,
}

use Direction::*;

#[derive(Debug, Clone)]
pub struct InvalidCardinal;

impl Display for InvalidCardinal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cardinal direction must be semantically valid.")
    }
}

impl Error for InvalidCardinal {}

// There was no real reason for me *not* to make a way to destructure a Cardinal3 to a String
// and regex it for validity. My head and hands hurt.
/// This validates a Cardinal3 - if valid it returns the cardinal. Else, an Error.
pub fn validate_cardinal3(direction: Cardinal3) -> Result<Cardinal3, InvalidCardinal> {
    let initial = direction.0;
    let ret: Result<Cardinal3, InvalidCardinal>;
    match initial {
        N | S => {
            // N... | S...
            match direction.1 {
                None => ret = Ok(direction), // N | S
                Some(second) => {
                    match second {
                        initial => {
                            // NN... | SS...
                            match direction.2 {
                                None => ret = Err(InvalidCardinal), // NN | SS
                                Some(last) => {
                                    match last {
                                        E | W => ret = Ok(direction),    // NNE | NNW |SSE |SSW
                                        _ => ret = Err(InvalidCardinal), // NNN | SSS
                                    }
                                }
                            }
                        }
                        E | W => {
                            // NE... | SE... | NW... | SW...
                            match direction.2 {
                                None => ret = Ok(direction),           // NE | SE | NW | SW |
                                Some(_) => ret = Err(InvalidCardinal), // NE... | SE... | NW... | SW...
                            }
                        }
                        _ => ret = Err(InvalidCardinal),
                    }
                }
            }
        }
        E | W => {
            // E... | S...
            match direction.1 {
                None => ret = Ok(direction), // E | W
                Some(second) => {
                    match second {
                        initial => ret = Err(InvalidCardinal), // EE... | WW...
                        N | S => {
                            match direction.2 {
                                None => ret = Err(InvalidCardinal), // EN | ES | WN | WS
                                Some(last) => {
                                    match last {
                                        initial => ret = Ok(direction),  // ESE | ENE | WSW | WNW
                                        _ => ret = Err(InvalidCardinal), // ESS | ESN | ESW | WSS | WSN | WSE
                                    }
                                }
                            }
                        }
                        _ => ret = Err(InvalidCardinal), // EW | WE
                    }
                }
            }
        }
    }
    ret
}
