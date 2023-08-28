use chrono::NaiveDate;

pub mod col_header;
pub mod row_headers;


pub fn str_to_date70(date: &str) -> Option<i64> {
    let date70 = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();

    return if let Ok(date) = NaiveDate::parse_from_str(date, "%Y%m%d") {
        Some(date.signed_duration_since(date70).num_days())
    } else {
        None
    }
}

pub fn int_to_date70(date: i32) -> Option<i64> {
    let date70 = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();

    let year = date / 10000;
    let month = (date - year * 10000) / 100;
    let day = date - year * 10000 - month * 100;

    let date_opt = NaiveDate::from_ymd_opt(year, month as u32, day as u32);

    return if let Some(date) = date_opt {
        Some(date.signed_duration_since(date70).num_days())
    } else {
        None
    }
}

