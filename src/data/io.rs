use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use crate::data::{Election, Region, Scenario};

// Opens an ElectoralSim format JSON and returns the Scenario contained inside
pub(crate) fn load(file_name: &str) -> Election {
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
        election.regions.insert(region.0.to_string(), Region::new(votes));
    }
    election
}

// Converts a Scenario to a ElectoralSim format JSON
pub(crate) fn save(scenario: Scenario, file_name: &str) {
    let mut json: serde_json::Value = serde_json::from_str("{}").expect("JSON was not well-formatted");
    json["id"] = serde_json::Value::String(scenario.id.clone());
    for election in &scenario.elections {
        for region in &election.regions {
            for candidate in &region.1.votes {
                json[region.0][candidate.0] = serde_json::Value::Number(serde_json::Number::from(candidate.1.clone()))
            }
        }
    }

    let mut file = File::create(file_name).unwrap();
    file.write_all(serde_json::to_string_pretty(&json).unwrap().as_bytes()).expect("TODO: panic message");
}

// Imports a properly-formatted CSV file as a single-election Scenario
pub(crate) fn import_file(file_name: &str) {
    // Rewrite from convert.rs and csv2json.py
}
