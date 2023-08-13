use crate::element::PointLogical;

struct ProjectRectangle {
    top_left: PointLogical,
    iw: i32,
    ih: i32,
    status: Status,
    left_line_mark: bool,
}

struct ProjectPolygon {
    i_points: Vec<PointLogical>,
    status: Status,
    left_line_mark: bool,
}

pub enum Status {
    Running,
    Finished,
    Waiting,
    Fault,
}
