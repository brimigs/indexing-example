

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct CandidateAccount {
    pub candidate_name: String,
    pub candidate_votes: u64,
}
