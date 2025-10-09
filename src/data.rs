use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Clone)]
pub(crate) struct Region {
    votes: HashMap<String, u32>
}

impl Region {
    // Constructor
    pub(crate) fn new(votes: HashMap<String, u32>) -> Region { Region{votes} }

    // Getter
    pub(crate) fn get_votes(&self, candidates: Vec<String>) -> u32 {
        let mut candidate_votes: u32 = 0;
        for candidate in &self.votes {
            if candidates.contains(&candidate.0) {
                candidate_votes += candidate.1;
            }
        }
        candidate_votes
    }
}

#[derive(Clone)]
pub(crate) struct Election {
    id: String,
    regions: HashMap<String, Region>
}

impl Election {
    // Constructor
    pub(crate) fn new(id: String) -> Election { Election{id, regions: HashMap::new()} }

    // Input/Output
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
            election.regions.insert(region.0.to_string(), Region::new(votes));
        }
        election
    }

    pub(crate) fn save_file(&self, file_name: &str) {
        let mut json: serde_json::Value = serde_json::from_str("{}").expect("JSON was not well-formatted");
        json["id"] = serde_json::Value::String(self.id.clone());
        for region in &self.regions {
            for candidate in &region.1.votes {
                json[region.0][candidate.0] = serde_json::Value::Number(serde_json::Number::from(candidate.1.clone()))
            }
        }
        
        let mut file = File::create(file_name).unwrap();
        file.write_all(serde_json::to_string_pretty(&json).unwrap().as_bytes()).expect("TODO: panic message");
    }

    // Getters
    pub(crate) fn get_votes_full(&self) -> HashMap<String, (u32, f32)> {
        let mut candidates: Vec<String> = Vec::new();
        for region in &self.regions {
            for candidate in &region.1.votes {
                if !candidates.contains(&candidate.0) {
                    candidates.push(candidate.0.to_string());
                }
            }
        }
        self.get_votes_partial(candidates)
    }
    pub(crate) fn get_votes_partial(&self, candidates: Vec<String>) -> HashMap<String, (u32, f32)> {
        let mut votes: HashMap<String, (u32, f32)> = HashMap::new();
        let mut total_votes: u32 = 0;

        // Sum votes for each candidate
        for region in &self.regions {
            for candidate in &region.1.votes {
                total_votes += candidate.1;
                if candidates.contains(&candidate.0) {
                    votes.entry(candidate.0.clone()).or_insert((candidate.1.clone(), 0.0)).0 += candidate.1;
                }
            }
        }

        // Calculate voteshare for each candidate
        for candidate in candidates {
            votes.entry(candidate.to_string()).or_default().1 += votes[&candidate].0.clone() as f32 / total_votes as f32;
        }
        votes
    }

    // Operations
    // RJay's method from YAPMS Discord:
    pub(crate) fn op_vote_split_full(&mut self, voteshare: HashMap<String, f32>) {
        // Get the current vote results for each candidate
        let pv_results = self.clone().get_votes_partial(voteshare.keys().cloned().collect());

        // Multiply by new and divide by old voteshares
        for region in &self.regions.clone() {
            let total_voteshare = region.1.clone().get_votes(voteshare.keys().cloned().collect());
            for candidate in &region.1.votes {
                if voteshare.contains_key(candidate.0) {
                    self.regions.get_mut(region.0).unwrap().votes.insert(candidate.0.to_string(), (candidate.1.clone() as f32 * voteshare[candidate.0]/pv_results[candidate.0].1) as u32);
                }
            }
            // Correct for voter turnout
            let voteshare_quotient = self.regions[region.0].clone().get_votes(voteshare.keys().cloned().collect()) as f32 / total_voteshare as f32;
            println!("Correcting with a voteshare quotient of {} = {} / {}", voteshare_quotient, self.regions[region.0].clone().get_votes(voteshare.keys().cloned().collect()), total_voteshare);
            for candidate in self.regions[region.0].votes.clone() {
                if voteshare.contains_key(&candidate.0) {
                    println!("{}", candidate.1);
                    self.regions.get_mut(region.0).unwrap().votes.insert(candidate.0.to_string(), (candidate.1.clone() as f32 * voteshare_quotient) as u32);
                }
            }
        }
    }
}
