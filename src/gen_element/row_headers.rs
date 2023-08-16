use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::element::{RowHeaderCell, RowHeaders};

pub fn from_devices(dl : &DeviceList) -> (RowHeaders, HashMap<String,i32>, Vec<i32>) {
    let mut row_index_map:HashMap<String,i32> = HashMap::new();

    let mut y_segments:Vec<i32> = vec![];

    let mut groups:Vec<RowHeaderCell> = vec![];
    let mut devices:Vec<RowHeaderCell> = vec![];

    for dg in &dl.groups {
        y_segments.push(dg.devices.len() as i32);
        groups.push(RowHeaderCell{
            ih: dg.devices.len() as i32,
            text: dg.name.clone(),
        });

        for d in &dg.devices {
            devices.push(RowHeaderCell{
                ih: 1,
                text: d.clone(),
            });
        }
    }

    let row_headers = RowHeaders{
        cols: vec![groups, devices],
    };

    return (row_headers, row_index_map, y_segments);
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceList{
    pub groups: Vec<DeviceGroup>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceGroup{
    pub name: String,
    pub devices: Vec<String>,
}

impl DeviceList{
    pub fn save_to_json(&self, path: &str){
        let json = serde_json::to_string_pretty(&self).unwrap();
        std::fs::write(path, json).expect("Unable to write file");
    }

    pub fn load_from_json(path: &str) -> DeviceList {
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        let dl = serde_json::from_reader(reader).unwrap();
        return dl;
    }

    pub fn expand_abbreviation(&self) -> DeviceList{
        let re = regex::Regex::new(r"^F(\d+)\~F?(\d+)$").unwrap();

        let mut result = DeviceList{
            groups: vec![],
        };

        for dg in &self.groups {
            let mut new_dg = DeviceGroup{
                name: dg.name.clone(),
                devices: vec![],
            };

            for d in &dg.devices{
                let name = d.to_uppercase();

                if let Some(caps) = re.captures(&name) {
                    println!("{}-{}", &caps[1], &caps[2]);
                    let start:i32 = caps[1].parse().unwrap();
                    let end :i32 = caps[2].parse().unwrap();

                    for i in start ..=end  {
                        new_dg.devices.push(format!("F{}", i));
                    }
                }else{
                    new_dg.devices.push(name.clone());
                }
            }
            result.groups.push(new_dg);
        }

        return result;
    }
}