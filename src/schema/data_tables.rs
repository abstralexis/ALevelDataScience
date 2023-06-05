use super::static_tables::*;
use crate::utils::uf32::Uf32;
use chrono::NaiveDate;

// TODO: Evaluate necessity for having all data as an `Option`

pub struct LocalData {
    pub location: Name, // TODO: split name enum
    pub date: NaiveDate,
    pub mean_air_temp_c: Uf32,
    pub total_rainfall_mm: Option<Uf32>,
    pub total_sunshine_hrs: Option<Uf32>, // TODO: constrain to 0..24
    pub mean_windspeed_kn: Option<u32>,
    pub mean_windspeed_bft: Option<Beaufort>,
    pub max_gust_kn: Option<u32>,
    pub max_humidity_percent: u32, // TODO: constrain to 0..=100
    pub mean_cloud_oktas: u32,     // TODO: constrain to 0..=8
    pub mean_visibility_dm: u32,   // TODO: evaluate possible constraint
    pub mean_pressure_hpa: u32,    // TODO: evaluate possible constraint
    pub mean_wind_dir_deg: u32,    // TODO: constrain to 0..=360
    pub mean_wind_dir_cardinal: Cardinal3,
    pub max_gust_dir_deg: u32, // TODO: constrain to 0..=360
    pub max_gust_dir_cardinal: Cardinal3,
}

pub struct OverseasData {
    pub location: Name, // TODO: split Name enum
    pub date: NaiveDate,
    pub mean_air_temp_c: Uf32,
    pub total_rainfall_mm: Option<Uf32>,
    pub mean_pressure_hpa: u32,
    pub mean_windspeed_kn: Option<u32>,
    pub mean_windspeed_bft: Option<Beaufort>,
}
