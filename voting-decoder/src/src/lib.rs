pub struct VotingDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub use instructions::{VotingDecoder as InstructionVotingDecoder, VotingInstruction};
pub use accounts::{VotingDecoder as AccountVotingDecoder, VotingAccount};