use crate::element::{Coordinate, Project, ProjectRect};

pub fn get_projects() -> Vec<Project> {
    let mut projects = vec![];

    projects.push(Project {
        id: String::from("001"),
        name: String::from("Project1"),
        rects: vec![
            ProjectRect::new2(Coordinate { x: 1, y: 0 }, &Coordinate { x: 9, y: 3 }),
        ],
        metering:vec![0.1,0.2,0.3,0.4,0.5,0.6,0.7,0.8,0.9,1.0,0.9,0.8,0.7,0.6,0.5],
    });

    projects.push(Project {
        id: String::from("002"),
        name: String::from("Project2"),
        rects: vec![
            ProjectRect::new2(Coordinate { x: 0, y: 4 }, &Coordinate { x: 6, y: 5 }),
        ],
        metering:vec![0.1,0.1,0.1, 1.0,1.0,1.0],
    });

    projects.push(Project {
        id: String::from("003"),
        name: String::from("Project3"),
        rects: vec![ProjectRect::new2(
            Coordinate { x: 2, y: 6 },
            &Coordinate { x: 10, y: 7 },
        )],
        metering:vec![0.1,0.1,0.5,1.0,1.0,1.0,0.8],
    });

    projects.push(Project {
        id: String::from("004"),
        name: String::from("Project4"),
        rects: vec![
            ProjectRect::new2(Coordinate { x: 3, y: 12 }, &Coordinate { x: 10, y: 15 }),
            ProjectRect::new2(Coordinate { x: 3, y: 16 }, &Coordinate { x: 7, y: 19 }),
        ],
        metering:vec![0.1,0.1,0.5,0.5,0.0,0.0,0.0,0.3],
    });



    return projects;
}