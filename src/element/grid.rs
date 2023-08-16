use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Table {
    pub col_headers: ColumnHeaders,
    pub row_groups: Vec<RowGroup>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RowGroup {
    pub header: RowHeaders,
    pub grid: Grid,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Grid {
    pub id: Option<String>,

    pub iw: i32,
    pub ih: i32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RowHeaders {
    pub cols: Vec<Vec<RowHeaderCell>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RowHeaderCell {
    pub ih: i32,
    pub text: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ColumnHeaders {
    pub rows: Vec<Vec<ColumnHeaderCell>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ColumnHeaderCell {
    pub iw: i32,
    pub text: String,
}
