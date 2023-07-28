
pub struct Grid{
    pub id: Option<String>,

    pub i_group: i32,
    pub ix: i32,
    pub iy: i32,
    pub iw: i32,
    pub ih: i32,
}

pub struct GridRowHeader{
    pub i_group: i32,
    pub iy: i32,
    pub row_headers: Vec<String>,
}

pub struct GridColumnHeader{
    pub rows: Vec<Vec<GridColumnHeaderCell>>,
}

pub struct GridColumnHeaderCell{
    pub iw: i32,
    pub text: String,
}