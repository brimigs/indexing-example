
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x6dfe7529e84aac2d")] 
pub struct PollAccount { 
        pub poll_name: String, 
        pub poll_description: String, 
        pub poll_voting_start: u64, 
        pub poll_voting_end: u64, 
        pub poll_option_index: u64, 
}