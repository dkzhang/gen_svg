use crate::element::{ProjectStatus, PROJECT_DELAYED, PROJECT_NORMAL, PROJECT_RUNNING, PROJECT_FAULT, PROJECT_EXPEDITED, PROJECT_QUEUED};
use crate::get_projects::project::{Project0, ProjectRect0};

pub fn mock_projects() -> Vec<Project0> {
    let mut projects = vec![];

    projects.push(Project0 {
        id: String::from("001"),
        name: String::from("项目A"),
        rects: vec![ProjectRect0 {
            date_from: 20230701,
            date_to: 20230703,
            devices: vec!["F303~F304".to_string()],
            status: ProjectStatus::new(vec![PROJECT_NORMAL, PROJECT_RUNNING]),
        }],
        metering: vec![
            0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5,
        ],
    });

    projects.push(Project0 {
        id: String::from("002"),
        name: String::from("项目B"),
        rects: vec![
            ProjectRect0 {
                date_from: 20230701,
                date_to: 20230707,
                devices: vec!["F305~F308".to_string()],
                status: ProjectStatus::new(vec![PROJECT_NORMAL, PROJECT_RUNNING]),
            },
            ProjectRect0 {
                date_from: 20230708,
                date_to: 20230709,
                devices: vec!["F306~F308".to_string()],
                status: ProjectStatus::new(vec![PROJECT_NORMAL, PROJECT_DELAYED]),
            },
        ],
        metering: vec![
            0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5,
        ],
    });

    projects.push(Project0 {
        id: String::from("003"),
        name: String::from("项目C"),
        rects: vec![ProjectRect0 {
            date_from: 20230701,
            date_to: 20230705,
            devices: vec!["F309~F310".to_string()],
            status: ProjectStatus::new(vec![PROJECT_NORMAL, PROJECT_RUNNING]),
        }],
        metering: vec![
            0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5,
        ],
    });

    projects.push(Project0 {
        id: String::from("004"),
        name: String::from("项目D"),
        rects: vec![
            ProjectRect0 {
                date_from: 20230701,
                date_to: 20230705,
                devices: vec!["F311~F313".to_string()],
                status: ProjectStatus::new(vec![PROJECT_NORMAL, PROJECT_RUNNING]),
            },
            ProjectRect0 {
                date_from: 20230706,
                date_to: 20230708,
                devices: vec!["F311~F313".to_string()],
                status: ProjectStatus::new(vec![PROJECT_NORMAL, PROJECT_DELAYED]),
            },
        ],
        metering: vec![
            0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5,
        ],
    });

    projects.push(Project0 {
        id: String::from("005"),
        name: String::from("项目F"),
        rects: vec![
            ProjectRect0 {
                date_from: 20230702,
                date_to: 20230709,
                devices: vec!["F314~F316".to_string()],
                status: ProjectStatus::new(vec![PROJECT_NORMAL, PROJECT_RUNNING]),
            },
            ProjectRect0 {
                date_from: 20230705,
                date_to: 20230709,
                devices: vec!["F317~F318".to_string()],
                status: ProjectStatus::new(vec![PROJECT_NORMAL, PROJECT_RUNNING]),
            },
        ],
        metering: vec![
            0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5,
        ],
    });

    projects.push(Project0 {
        id: String::from("Fault001"),
        name: String::from(""),
        rects: vec![
            ProjectRect0 {
                date_from: 20230702,
                date_to: 20230704,
                devices: vec!["F317~F318".to_string()],
                status: ProjectStatus::new(vec![PROJECT_NORMAL, PROJECT_FAULT]),
            },
        ],
        metering: vec![
            0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5,
        ],
    });

    projects.push(Project0 {
        id: String::from("006"),
        name: String::from("项目H"),
        rects: vec![
            ProjectRect0 {
                date_from: 20230709,
                date_to: 20230713,
                devices: vec!["F309~F313".to_string()],
                status: ProjectStatus::new(vec![PROJECT_NORMAL, PROJECT_RUNNING]),
            },
        ],
        metering: vec![
            0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5,
        ],
    });

    projects.push(Project0 {
        id: String::from("007"),
        name: String::from("项目I"),
        rects: vec![
            ProjectRect0 {
                date_from: 20230710,
                date_to: 20230716,
                devices: vec!["F314~F318".to_string()],
                status: ProjectStatus::new(vec![PROJECT_EXPEDITED, PROJECT_QUEUED]),
            },
        ],
        metering: vec![
            0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5,
        ],
    });

    projects.push(Project0 {
        id: String::from("008"),
        name: String::from("项目G"),
        rects: vec![
            ProjectRect0 {
                date_from: 20230710,
                date_to: 20230718,
                devices: vec!["F303~F308".to_string()],
                status: ProjectStatus::new(vec![PROJECT_NORMAL, PROJECT_QUEUED]),
            },
        ],
        metering: vec![
            0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5,
        ],
    });

    projects.push(Project0 {
        id: String::from("009"),
        name: String::from("项目J"),
        rects: vec![
            ProjectRect0 {
                date_from: 20230714,
                date_to: 20230721,
                devices: vec!["F309~F313".to_string()],
                status: ProjectStatus::new(vec![PROJECT_NORMAL, PROJECT_QUEUED]),
            },
            ProjectRect0 {
                date_from: 20230719,
                date_to: 20230721,
                devices: vec!["F306~F308".to_string()],
                status: ProjectStatus::new(vec![PROJECT_NORMAL, PROJECT_QUEUED]),
            },
        ],
        metering: vec![
            0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5,
        ],
    });

    projects.push(Project0 {
        id: String::from("010"),
        name: String::from("项目L"),
        rects: vec![
            ProjectRect0 {
                date_from: 20230717,
                date_to: 20230723,
                devices: vec!["F314~F318".to_string()],
                status: ProjectStatus::new(vec![PROJECT_NORMAL, PROJECT_QUEUED]),
            },
        ],
        metering: vec![
            0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5,
        ],
    });

    return projects;

    // projects.push(Project {
    //     id: String::from("001"),
    //     name: String::from("Project1"),
    //     rects: vec![ProjectRect::new2(
    //         Coordinate { x: 1, y: 0 },
    //         &Coordinate { x: 9, y: 3 },
    //         ProjectStatus::new(vec![PROJECT_NORMAL,PROJECT_RUNNING]),
    //     )],
    //     metering: vec![
    //         0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5,
    //     ],
    // });
    //
    // projects.push(Project {
    //     id: String::from("002"),
    //     name: String::from("Project2"),
    //     rects: vec![ProjectRect::new2(
    //         Coordinate { x: 0, y: 4 },
    //         &Coordinate { x: 6, y: 5 },
    //         ProjectStatus::new(vec![PROJECT_NORMAL,PROJECT_HISTORICAL]),
    //     )],
    //     metering: vec![0.1, 0.1, 0.1, 1.0, 1.0, 1.0],
    // });
    //
    // projects.push(Project {
    //     id: String::from("003"),
    //     name: String::from("Project3"),
    //     rects: vec![ProjectRect::new2(
    //         Coordinate { x: 2, y: 6 },
    //         &Coordinate { x: 10, y: 7 },
    //         ProjectStatus::new(vec![PROJECT_EXPEDITED, PROJECT_QUEUED]),
    //     )],
    //     metering: vec![0.1, 0.1, 0.5, 1.0, 1.0, 1.0, 0.8],
    // });
    //
    // projects.push(Project {
    //     id: String::from("004"),
    //     name: String::from("Project4"),
    //     rects: vec![
    //         ProjectRect::new2(
    //             Coordinate { x: 3, y: 12 },
    //             &Coordinate { x: 10, y: 15 },
    //             ProjectStatus::new(vec![PROJECT_NORMAL,PROJECT_RUNNING]),
    //         ),
    //         ProjectRect::new2(
    //             Coordinate { x: 3, y: 16 },
    //             &Coordinate { x: 7, y: 19 },
    //             ProjectStatus::new(vec![PROJECT_NORMAL,PROJECT_RUNNING]),
    //         ),
    //     ],
    //     metering: vec![0.1, 0.1, 0.5, 0.5, 0.0, 0.0, 0.0, 0.3],
    // });
    //
    // return projects;
}
