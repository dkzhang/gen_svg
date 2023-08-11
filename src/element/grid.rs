
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
    pub cols: Vec<Vec<RowHeaderCell>>,
}

pub struct RowHeaderCell {
    pub ih: i32,
    pub text: String,
}
pub struct ColumnHeader {
    pub rows: Vec<Vec<ColumnHeaderCell>>,
}

pub struct ColumnHeaderCell {
    pub iw: i32,
    pub text: String,
}