use std::collections::HashMap;
use crate::element::{ColumnHeaders, ColumnHeaderCell};
use chrono::{NaiveDate, Duration, Datelike};

pub fn from_date70(start: i64, end: i64) -> (ColumnHeaders, Vec<i32>) {
    let date70 = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();

    let mut day_row:Vec<ColumnHeaderCell> = vec![];
    let mut month_row:Vec<ColumnHeaderCell> = vec![];
    let mut cm = date70 + Duration::days(start);
    let mut cm_days = 0;

    let mut col_index_map:HashMap<i64,i32> = HashMap::new();

    for i in start..=end {
        let current_date = date70 + Duration::days(i);

        day_row.push(ColumnHeaderCell{
            iw: 1,
            text: current_date.format("%d").to_string(),
        });

        col_index_map.insert(i, (i-start)  as i32);

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
        col_index_map,
    },vec![(end - start + 1) as i32]);
}