//! This module represents the types for the values in
//! the static tables of the database, e.g. unit names to
//! for optimisation using foreign integer keys as integers
//! are smaller than VarChars.

use std::{error::Error, fmt::Display};
use regex::Regex;
use anyhow;
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

/// Converts a Cardinal3 to a String
fn c3_to_string(direction: Cardinal3) -> String {
    let mut string = String::new();     // Container for string
    string.push_str(&direction.0.to_string());    // Push first item
    // Handle the optional directions
    match direction.1 {
        None => return string,
        Some(s) => {
            string.push_str(&s.to_string());
            match direction.2 {
                None => return string,
                Some(s) => string.push_str(&s.to_string()),
            }
        }
    }
    string
}

/// The cardinal direction enum, for NSEW.
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    N,
    S,
    E,
    W,
}

use Direction::*;

impl ToString for Direction {
    fn to_string(&self) -> String {
        match *self {
            N => "N".to_string(),
            S => "S".to_string(),
            E => "E".to_string(),
            W => "W".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InvalidCardinal;

impl Display for InvalidCardinal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cardinal direction must be semantically valid.")
    }
}

impl Error for InvalidCardinal {}

/// This validates a Cardinal3 - if valid it returns the cardinal. Else, an Error.
pub fn validate_cardinal3(direction: Cardinal3) -> anyhow::Result<Cardinal3> {
    // I lost my nice generic regex notes so this will do for now.
    match match direction.0 {
        // Using anyhow because I don't want to deal with Box<dyn std::error::Error>
        // and calling ? on the regex is just to satisfy the compiler, it's known to work.
        N => Regex::new("^N([EW]|N[EW])?")?.is_match(&c3_to_string(direction)),
        S => Regex::new("^S([EW]|S[EW])?")?.is_match(&c3_to_string(direction)),
        E => Regex::new("^E([NS]E)?")?.is_match(&c3_to_string(direction)),
        W => Regex::new("^W([NS]W)?")?.is_match(&c3_to_string(direction)),
    } {
        true => Ok(direction),
        false => Err(InvalidCardinal.into())
    }
}
