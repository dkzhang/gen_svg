//
// struct line {
//     from: (i32, i32),
//     to: (i32, i32),
//     stroke: String,
//     stroke_width: i32,
// }
//
// struct i_rect {
//     top_left: (i32, i32),
//     width: i32,
//     height: i32,
// }
//
//
//
// struct i_grid {
//     rows: i32,
//     columns: i32,
//     row_heads: Vec<String>,
// }
//
// struct point {
//     x: i32,
//     y: i32,
// }
//
// fn parse_grid(
//     ig: &i_grid,
//     s: &style,
//     top_left: &point,
//     head_width: i32,
//     cell_height: i32,
//     cell_width: i32,
// ) -> grid {
//     let grid_width = head_width + cell_width * ig.columns;
//     let grid_height = cell_height * ig.rows;
//
//     let frame = rectangle {
//         top_left: (top_left.x, top_left.y),
//         bottom_right: (
//             top_left.x + grid_width as i32,
//             top_left.y + grid_height as i32,
//         ),
//         style: s.clone(),
//     };
//
//     let mut h_lines: Vec<line> = Vec::new();
//     for i in 0..ig.rows {
//         let y = top_left.y + i * cell_height;
//         let line = line {
//             from: (top_left.x, y),
//             to: (top_left.x + grid_width as i32, y),
//             stroke: s.stroke.clone(),
//             stroke_width: s.stroke_width,
//         };
//         h_lines.push(line);
//     }
//
//     let mut v_lines: Vec<line> = Vec::new();
//     for i in 0..ig.columns {
//         let x = top_left.x + head_width + i * cell_width;
//         let line = line {
//             from: (x, top_left.y),
//             to: (x, top_left.y + grid_height as i32),
//             stroke: s.stroke.clone(),
//             stroke_width: s.stroke_width,
//         };
//         v_lines.push(line);
//     }
//
//     let grid = grid {
//         frame,
//         h_lines,
//         v_lines,
//         style: s.clone(),
//     };
//
//     return grid;
// }
pub mod grid;
pub mod project;

pub use grid::*;
pub use project::*;
