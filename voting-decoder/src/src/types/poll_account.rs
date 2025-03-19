

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct PollAccount {
    pub poll_name: String,
    pub poll_description: String,
    pub poll_voting_start: u64,
    pub poll_voting_end: u64,
    pub poll_option_index: u64,
}
