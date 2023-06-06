use super::static_tables::*;
use crate::utils::{float_hours::DayHours, oktas::Oktas, percentage::UPercent100, uf32::Uf32};
use chrono::NaiveDate;

/// The `LocationData` trait has a method for getting
/// the location out of data. It is used to group the
/// data when needed.
pub trait LocationData {
    /// Location data must have an attribute that impls
    /// Name, and must return a Box of it. Box is used
    /// as the size of dyn Name is not known at compile time.
    fn get_location(&self) -> Box<dyn Name>;
}

/// `LocalData` implements `LocationData`, and can take
/// only a `LocalName` for the name. Represents the data
/// stored for local (UK) locations.
pub struct LocalData {
    pub location: LocalName,
    pub date: NaiveDate,
    pub mean_air_temp_c: Option<Uf32>,
    pub total_rainfall_mm: Option<Uf32>,
    pub total_sunshine_hrs: Option<DayHours>,
    pub mean_windspeed_kn: Option<u32>,
    pub mean_windspeed_bft: Option<Beaufort>,
    pub max_gust_kn: Option<u32>,
    pub max_humidity_percent: Option<UPercent100>,
    pub mean_cloud_oktas: Option<Oktas>,
    pub mean_visibility_dm: Option<u32>,
    pub mean_pressure_hpa: Option<u32>,
    pub mean_wind_dir_deg: Option<u32>, // TODO: constrain to 0..=360
    pub mean_wind_dir_cardinal: Option<Cardinal3>,
    pub max_gust_dir_deg: Option<u32>, // TODO: constrain to 0..=360
    pub max_gust_dir_cardinal: Option<Cardinal3>,
}
impl LocationData for LocalData {
    fn get_location(&self) -> Box<dyn Name> {
        Box::new(self.location.clone())
    }
}

/// `OverseasData` implements `LocationData` and can only
/// take an `OverseasName` as the name. Represents the data
/// stored for Overseas locations.
pub struct OverseasData {
    pub location: OverseasName,
    pub date: NaiveDate,
    pub mean_air_temp_c: Option<Uf32>,
    pub total_rainfall_mm: Option<Uf32>,
    pub mean_pressure_hpa: Option<u32>,
    pub mean_windspeed_kn: Option<u32>,
    pub mean_windspeed_bft: Option<Beaufort>,
}
impl LocationData for OverseasData {
    fn get_location(&self) -> Box<dyn Name> {
        Box::new(self.location.clone())
    }
}
