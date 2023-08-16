use std::collections::HashMap;
use crate::element::{ColumnHeaders, ColumnHeaderCell, RowHeaders};
use chrono::{NaiveDate, Duration, Datelike};
use chrono::format::Fixed::ShortMonthName;

pub fn from_date(start: &str, end: &str) -> (ColumnHeaders,HashMap<String,i32>, Vec<i32>) {
    let start_date = NaiveDate::parse_from_str(start, "%Y%m%d").expect("Failed to parse date");
    let end_date = NaiveDate::parse_from_str(end, "%Y%m%d").expect("Failed to parse date");

    let duration = end_date.signed_duration_since(start_date);

    let mut day_row:Vec<ColumnHeaderCell> = vec![];
    let mut month_row:Vec<ColumnHeaderCell> = vec![];
    let mut cm = start_date;
    let mut cm_days = 0;

    let mut col_index_map:HashMap<String,i32> = HashMap::new();

    for i in 0..=duration.num_days() {
        let current_date = start_date + Duration::days(i);

        day_row.push(ColumnHeaderCell{
            iw: 1,
            text: current_date.format("%d").to_string(),
        });

        col_index_map.insert(current_date.format("%Y%m%d").to_string(), i as i32);

        if current_date.month() != cm.month() {
            month_row.push(ColumnHeaderCell{
                iw: cm_days,
                text: cm.format("%b").to_string(),
            });

            cm = current_date;
            cm_days = 1;
        }else{
            cm_days += 1;
        }
    }
    // push the last month
    month_row.push(ColumnHeaderCell{
        iw: cm_days,
        text: cm.format("%b").to_string(),
    });

    return (ColumnHeaders {
        rows: vec![month_row, day_row],
    }, col_index_map, vec![duration.num_days() as i32 + 1]);
}