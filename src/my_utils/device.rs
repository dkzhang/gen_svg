use serde::{Deserialize, Serialize};

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

    pub fn expand_devices(&self) -> DeviceList{
        let mut result = DeviceList{
            groups: vec![],
        };

        for dg in &self.groups {
            let new_dg = DeviceGroup{
                name: dg.name.clone(),
                devices: expand_abbreviation(&dg.devices),
            };

            result.groups.push(new_dg);
        }
        return result;
    }
}

pub fn expand_abbreviation(devices: &Vec<String>) -> Vec<String>{
    let re = regex::Regex::new(r"^F(\d+)\~F?(\d+)$").unwrap();

    let mut result = vec![];

    for d in devices{
        let name = d.to_uppercase();

        if let Some(caps) = re.captures(&name) {
            let start:i32 = caps[1].parse().unwrap();
            let end :i32 = caps[2].parse().unwrap();

            for i in start ..=end  {
                result.push(format!("F{}", i));
            }
        }else{
            result.push(name.clone());
        }
    }

    return result;
}