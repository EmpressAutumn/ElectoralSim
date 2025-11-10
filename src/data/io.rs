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
pub(crate) fn import(path: std::path::PathBuf) -> Scenario {
    let file = path.file_stem().and_then(|s| s.to_str()) .unwrap_or("").to_string();
    let mut regions: HashMap<String, Region> = HashMap::new();

    /*
    election = {
        "id": file_name,
        "regions": {}
    }

    election_dict = {}

    with open(f"{file_name}.csv", mode='r') as file:
        csv_reader = csv.reader(file)
        headers = next(csv_reader)
        for row in csv_reader:
            key = row[0]
            election_dict[key] = {headers[i]: row[i] for i in range(1, len(headers))}

    for state, results in election_dict.items():
        state_results = {}
        for candidate, votes in results.items():
            state_results[candidate] = int(votes.replace(',', ''))
        election["regions"][state] = state_results

    json.dump(election, open(f"{file_name}.json", "w"))
    */

    Scenario {
        name: file,
        elections: Vec::from([
            Election {
                date: "0000-00-00".to_string(),
                races: Vec::from([
                    Race {
                        id: "unknown".to_string(),
                        regions
                    }
                ])
            }
        ])
    }
}
