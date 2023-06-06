//! This is schema file defining the static tables for my database. These
//! will include the Location Names, Beaufort Unit names, and Cardinal Directions as
//! they are used by the tables and are static - they hold only a few hard-coded values,
//! and I can model these as enums for better type handling for the functions added later.

use diesel::connection::SimpleConnection;
use diesel::deserialize::{self, FromSql};
use diesel::sqlite::{Sqlite, SqliteValue, SqliteType};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::*;

#[derive(SqlType)]
#[diesel(sqlite_type(name = "Text"))]
pub struct Location;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type = Location)]
pub enum LocationNames {
    Beijing,
    Camborne,
    Heathrow,
    Hurn,
    Jacksonville,
    Leeming,
    Leuchars,
    Perth,
}

impl ToSql<Location, Sqlite> for LocationNames {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        use LocationNames::*;

        match *self {
            Beijing => out.set_value(String::from("foo")),
            Camborne => out.set_value(String::from("bar")),
            Heathrow => out.set_value(String::from("heathrow")),
            Hurn => out.set_value(String::from("hurn")),
            Jacksonville => out.set_value(String::from("jacksonville")),
            Leeming => out.set_value(String::from("leeming")),
            Leuchars => out.set_value(String::from("leuchars")),
            Perth => out.set_value(String::from("perth")),
        }
        Ok(IsNull::No)
    }
}

// impl FromSql<Location, Sqlite> for LocationNames {
//     fn from_sql(value: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
//         use LocationNames::*;

        

//         match value.into() {
//             "foo" => Ok(Beijing),
//             "bar" => Ok(Camborne),
//             "heathrow" => Ok(Heathrow),
//             "hurn" => Ok(Hurn),
//             "jacksonville" => Ok(Jacksonville),
//             "leeming" => Ok(Leeming),
//             "leuchars" => Ok(Leuchars),
//             "perth" => Ok(Perth),
//             _ => Err("Unrecognised location name".into())
//         }
//     }
// } 