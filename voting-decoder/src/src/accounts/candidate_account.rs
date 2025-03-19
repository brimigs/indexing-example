
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x45cb492bcbaa6079")] 
pub struct CandidateAccount { 
        pub candidate_name: String, 
        pub candidate_votes: u64, 
}