pub mod project;
mod mock_projects;

use std::collections::HashMap;
use crate::element::{Coordinate, Project, ProjectRect, ProjectStatus, PROJECT_NORMAL, PROJECT_RUNNING, PROJECT_HISTORICAL, PROJECT_EXPEDITED, PROJECT_QUEUED};
use crate::get_projects::project::Project0;
use crate::my_utils::date::Date70;

pub fn get_projects0() -> Vec<Project0> {
    return mock_projects::mock_projects();
}
pub fn get_projects(
    row_index_map: &HashMap<String, i32>,
    col_index_map: &HashMap<Date70, i32>,
) -> Vec<Project> {
    return Project::new_vec(&mock_projects::mock_projects(), row_index_map, col_index_map);
}
