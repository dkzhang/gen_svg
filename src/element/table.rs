use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Table {
    pub col_headers: ColumnHeaders,
    pub row_headers: RowHeaders,
    pub grid: Grid,
}
// #[derive(Serialize, Deserialize, Debug)]
// pub struct RowGroup {
//     pub header: RowHeaders,
//     pub grid: Grid,
// }
#[derive(Serialize, Deserialize, Debug)]
pub struct Grid {
    pub id: Option<String>,

    pub x_segments: Vec<i32>,
    pub y_segments: Vec<i32>,
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

impl Table {
    pub fn save_to_json(&self, path: &str) {
        let json = serde_json::to_string_pretty(&self).unwrap();
        std::fs::write(path, json).expect("Unable to write file");
    }

    pub fn load_from_json(path: &str) -> Table {
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        let dl = serde_json::from_reader(reader).unwrap();
        return dl;
    }
}
