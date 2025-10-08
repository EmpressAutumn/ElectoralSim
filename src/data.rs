use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct Region {
    id: String,
    votes: HashMap<String, u32>
}

impl Region {
    // Constructor
    pub(crate) fn new(id: String, votes: HashMap<String, u32>) -> Region { Region{id, votes} }

    // Getter
    pub(crate) fn get_votes(self, candidates: Vec<String>) -> u32 {
        let mut candidate_votes: u32 = 0;
        for candidate in self.votes {
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
    pub regions: Vec<Region>
}

impl Election {
    // Constructor
    pub(crate) fn new(id: String) -> Election { Election{id, regions: vec![]} }

    pub(crate) fn add_region(&mut self, region: Region) {
        self.regions.push(region);
    }

    // Getters
    pub(crate) fn get_votes_full(self) -> HashMap<String, (u32, f32)> {
        let mut candidates: Vec<String> = Vec::new();
        for region in &self.regions {
            for candidate in &region.votes {
                if !candidates.contains(&candidate.0) {
                    candidates.push(candidate.0.to_string());
                }
            }
        }
        self.get_votes_partial(candidates)
    }
    pub(crate) fn get_votes_partial(self, candidates: Vec<String>) -> HashMap<String, (u32, f32)> {
        let mut votes: HashMap<String, (u32, f32)> = HashMap::new();
        let mut total_votes: u32 = 0;

        // Sum votes for each candidate
        for region in self.regions {
            for candidate in region.votes {
                total_votes += candidate.1;
                if candidates.contains(&candidate.0) {
                    votes.entry(candidate.0).or_insert((candidate.1, 0.0)).0 += candidate.1;
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
        for (i, region) in self.regions.clone().iter().enumerate() {
            let total_voteshare = region.clone().get_votes(voteshare.keys().cloned().collect());
            for candidate in &region.votes {
                if voteshare.contains_key(candidate.0) {
                    self.regions[i].votes.insert(candidate.0.to_string(), (candidate.1.clone() as f32 * voteshare[candidate.0]/pv_results[candidate.0].1) as u32);
                }
            }
            // Correct for voter turnout
            let voteshare_quotient = self.regions[i].clone().get_votes(voteshare.keys().cloned().collect()) as f32 / total_voteshare as f32;
            println!("Correcting with a voteshare quotient of {} = {} / {}", voteshare_quotient, self.regions[i].clone().get_votes(voteshare.keys().cloned().collect()), total_voteshare);
            for candidate in self.regions[i].votes.clone() {
                if voteshare.contains_key(&candidate.0) {
                    println!("{}", candidate.1);
                    self.regions[i].votes.insert(candidate.0.to_string(), (candidate.1.clone() as f32 * voteshare_quotient) as u32);
                }
            }
        }
    }
}
