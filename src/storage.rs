use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub total_distance: f64,
}

// Loads total distance from a JSON file
pub fn load_distance() -> f64 {
    let mut file =
        File::open("distance.json").unwrap_or_else(|_| File::create("distance.json").unwrap());
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    if content.is_empty() {
        return 0.0;
    }

    let data: Data = serde_json::from_str(&content).unwrap_or(Data {
        total_distance: 0.0,
    });
    data.total_distance
}

// Saves the accumulated distance in a JSON file
pub fn save_distance(distance: f64) {
    let data = Data {
        total_distance: distance,
    };
    let json = serde_json::to_string_pretty(&data).unwrap();

    let mut file = File::create("distance.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
