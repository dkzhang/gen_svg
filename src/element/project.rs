struct ProjectRectangle{
    ix: i32,
    iy: i32,
    iw: i32,
    ih: i32,
    status: Status,
    left_line_mark: bool,
}

struct ProjectPolygon{
    i_points: Vec<(i32, i32)>,
    status: Status,
    left_line_mark: bool,
}

pub enum Status{
    Running,
    Finished,
    Waiting,
    Fault,
}