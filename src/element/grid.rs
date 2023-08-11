
pub struct Table{
    pub col_headers: ColumnHeader,
    pub row_groups: Vec<RowGroup>,
}

pub struct RowGroup{
    pub header: RowHeader,
    pub grid: Grid,
}

pub struct Grid{
    pub id: Option<String>,

    pub iw: i32,
    pub ih: i32,
}

pub struct RowHeader {
    pub i_group: i32,
    pub iy: i32,
    pub row_headers: Vec<String>,
}

pub struct ColumnHeader {
    pub rows: Vec<Vec<ColumnHeaderCell>>,
}

pub struct ColumnHeaderCell {
    pub iw: i32,
    pub text: String,
}