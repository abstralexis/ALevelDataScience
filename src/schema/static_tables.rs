//! This module represents the types for the values in
//! the static tables of the database, e.g. unit names to
//! for optimisation using foreign integer keys as integers
//! are smaller than VarChars.

/// Represents the value in the database that is stored
/// for the location of the data. This means that the data
/// will be stored in one large table but with a foreign
/// key to it's location name. This is to minimise data
/// duplication, as we are not implementing a new table
/// for each location although it may be more normalised.
pub struct LocationName {
    /// The **primary key**
    id: u32,
    /// The name for the location. This is required,
    /// hence it is `Name` and not `Option<Name>`.
    name: Name
}

/// This enum represents the names of locations that are
/// in the LDS. This is primarily accessed through use of
/// the `LocationName` struct.
pub enum Name {
    Beijing,
    Camborne,
    Heathrow,
    Hurn,
    Jacksonville,
    Leeming,
    Leuchars,
    Perth,
}

/// The Beaufort Unit type for the database. Holds a `u8`
/// for the **primary key** (`id`), and an `Option` for 
/// the Beaufort Scale conversion enum. 
pub struct BeaufortUnit {
    /// The **primary key**
    pub id: u32,
    /// `Option` models the table's "n/a" values as 
    /// `Option::None`, and the breeze conversions are 
    /// `Some<Beaufort>`.
    pub unit: Option<Beaufort>,
}

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

/// The cardinal direction type. Models `r#[NSEW]{1, 3}`.
/// This does **NOT** exclude logically invalid directions.
pub type Cardinal3 = (Direction, Option<Direction>, Option<Direction>);

/// This is the struct to represent the CardinalDirection
/// table. It has an id **primary key** and a direction.
/// This direction is composed of a tuple, containing at least
/// one primary cardinal direction, and then another two optionals.
/// This represents variants like N, SW, and ESE for example.
/// 
/// Note: **This does _NOT_ account for possible logically invalid
/// directions such as: EEE, NS, WEN, etc.** For now, for simplicity,
/// I am forgoing this possibility of invalid state as I think we can
/// safely assume that the directions provided in the LDS are valid.
pub struct CardinalDirection {
    /// The **primary key**
    pub id: u32,

    /// Use the Cardinal3 direction type.
    direction: Cardinal3,
}

/// The cardinal direction enum, for NSEW.
pub enum Direction {
    N,
    S,
    E,
    W,
} 

// TODO: add some function for safely constructing a Cardinal3