use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::data::{Election, Region};

pub(crate) fn load_file(file_name: &str) -> Election {
    let mut file = File::open(file_name).unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();

    let json: serde_json::Value = serde_json::from_str(file_contents.as_str()).expect("JSON was not well-formatted");
    let mut election = Election::new(json["id"].as_str().unwrap().to_string());
    for region in json["regions"].as_object().unwrap().iter() {
        let mut votes: HashMap<String, u32> = HashMap::new();
        for candidate in region.1.as_object().unwrap().iter() {
            votes.insert(candidate.0.to_string(), candidate.1.as_number().unwrap().as_u128().unwrap() as u32);
        }
        election.add_region(Region::new(
            region.0.as_str().to_string(),
            votes
        ));
    }
    election
}
