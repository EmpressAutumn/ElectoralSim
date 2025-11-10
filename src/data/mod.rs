pub(crate) mod io;

use std::collections::HashMap;
use std::io::{Read, Write};

#[derive(Clone)]
pub(crate) struct Region {
    votes: HashMap<String, u32>
}

impl Region {
    // Constructor
    pub(crate) fn new(votes: HashMap<String, u32>) -> Region { Region {votes} }

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
pub(crate) struct Race {
    id: String,
    regions: HashMap<String, Region>
}

impl Race {
    // Constructor
    pub(crate) fn new(id: String) -> Race { Race {id, regions: HashMap::new()} }

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
        for candidate in &candidates {
            votes.insert(candidate.clone(), (0, 0.0));
        }

        // Sum votes for each candidate
        for region in &self.regions {
            for candidate in &region.1.votes {
                if candidates.contains(&candidate.0) {
                    total_votes += candidate.1;
                    votes.get_mut(candidate.0).unwrap().0 += candidate.1;
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
            let voteshare_quotient = total_voteshare as f32 / self.regions[region.0].clone().get_votes(voteshare.keys().cloned().collect()) as f32;
            for candidate in self.regions[region.0].votes.clone() {
                if voteshare.contains_key(&candidate.0) {
                    self.regions.get_mut(region.0).unwrap().votes.insert(candidate.0.to_string(), (candidate.1.clone() as f32 * voteshare_quotient) as u32);
                }
            }
        }
    }
}

#[derive(Clone)]
pub(crate) struct Election {
    date: String,
    races: Vec<Race>
}

impl Election {
    pub(crate) fn new(date: String) -> Election {
        Election {
            date,
            races: Vec::new()
        }
    }
    pub(crate) fn date(&self) -> &str { &self.date }
}

#[derive(Clone)]
pub(crate) struct Scenario {
    name: String,
    elections: Vec<Election>
}

impl Scenario {
    pub(crate) fn default() -> Scenario {
        Scenario {
            name: "default".to_string(),
            elections: Vec::new()
        }
    }
    
    pub(crate) fn new(name: String) -> Scenario {
        Scenario {
            name,
            elections: Vec::new()
        }
    }
    
    pub(crate) fn elections(&self) -> &Vec<Election> { &self.elections }
}
