use std::collections::HashMap;
use crate::element::{RowHeaderCell, RowHeaders};
use crate::my_utils::device::DeviceList;

pub fn from_devices(dl : &DeviceList) -> (RowHeaders, Vec<i32>){
    let mut row_index_map:HashMap<String,i32> = HashMap::new();

    let mut y_segments:Vec<i32> = vec![];

    let mut groups:Vec<RowHeaderCell> = vec![];
    let mut devices:Vec<RowHeaderCell> = vec![];

    let mut current_index = 0;

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

            row_index_map.insert(d.clone(), current_index);
            current_index += 1;
        }
    }

    return (RowHeaders{
        cols: vec![groups, devices],
        row_index_map,
    }, y_segments);
}
