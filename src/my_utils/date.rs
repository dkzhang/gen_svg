use chrono::NaiveDate;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub static DATE70: Lazy<NaiveDate> = Lazy::new(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct DateInt(i32); // date integer like 20200101

pub type DateInt = i32;
pub type Date70 = i64;

pub fn str_to_date70(date: &str) -> Option<Date70> {
    return if let Ok(date) = NaiveDate::parse_from_str(date, "%Y%m%d") {
        Some(date.signed_duration_since(*DATE70).num_days())
    } else {
        None
    };
}

pub fn int_to_date70(date: DateInt) -> Option<Date70> {
    let year = date / 10000;
    let month = (date - year * 10000) / 100;
    let day = date - year * 10000 - month * 100;

    let date_opt = NaiveDate::from_ymd_opt(year, month as u32, day as u32);

    return if let Some(date) = date_opt {
        Some(date.signed_duration_since(*DATE70).num_days())
    } else {
        None
    };
}

pub fn date70_to_date(date70: Date70) -> NaiveDate {
    *DATE70 + chrono::Duration::days(date70)
}
