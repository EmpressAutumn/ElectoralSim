use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use crate::data::{Election, Race, Region, Scenario};

// Opens an ElectoralSim format JSON and returns the Scenario contained inside
pub(crate) fn open(file_name: std::path::PathBuf) -> Scenario {
    let mut file = File::open(file_name).unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();

    let json: serde_json::Value = serde_json::from_str(file_contents.as_str()).expect("JSON was not well-formatted");
    let mut scenario = Scenario::new(json["name"].to_string());
    for (date, election) in json["elections"].as_object().unwrap().iter() {
        let mut election_obj = Election::new(date.clone());
        for (id, race) in election["races"].as_object().unwrap().iter() {
            let mut race_obj = Race::new(id.clone());
            for (region, results) in race["regions"].as_object().unwrap().iter() {
                let mut votes: HashMap<String, u32> = HashMap::new();
                for (name, candidate) in results.as_object().unwrap().iter() {
                    votes.insert(name.clone(), candidate.as_number().unwrap().as_u128().unwrap() as u32);
                }
                race_obj.regions.insert(region.clone(), Region::new(votes));
            }
            election_obj.races.push(race_obj);
        }
        scenario.elections.push(election_obj);
    }

    let mut race = Race::new(json["id"].as_str().unwrap().to_string());
    for region in json["regions"].as_object().unwrap().iter() {
        let mut votes: HashMap<String, u32> = HashMap::new();
        for candidate in region.1.as_object().unwrap().iter() {
            votes.insert(candidate.0.to_string(), candidate.1.as_number().unwrap().as_u128().unwrap() as u32);
        }
        race.regions.insert(region.0.to_string(), Region::new(votes));
    }
    scenario
}

// Converts a Scenario to a ElectoralSim format JSON
pub(crate) fn save(scenario: Scenario, file_name: std::path::PathBuf) {
    let mut json: serde_json::Value = serde_json::from_str("{}").expect("JSON was not well-formatted");
    json["id"] = serde_json::Value::String(scenario.name.clone());
    for election in &scenario.elections {
        for race in &election.races {
            for region in &race.regions {
                for candidate in &region.1.votes {
                    json[region.0][candidate.0] = serde_json::Value::Number(serde_json::Number::from(candidate.1.clone()))
                }
            }
        }
    }

    let mut file = File::create(file_name).unwrap();
    file.write_all(serde_json::to_string_pretty(&json).unwrap().as_bytes()).expect("TODO: panic message");
}

// Imports a properly-formatted CSV file as a single-election Scenario
pub(crate) fn import_file(file_name: std::path::PathBuf) {
    // Rewrite from convert.rs and csv2json.py
}
