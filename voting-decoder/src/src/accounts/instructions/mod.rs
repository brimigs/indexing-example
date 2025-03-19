



pub use super::VotingDecoder;
pub mod initialize_candidate;
pub mod initialize_poll;
pub mod vote;

#[derive(carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum VotingInstruction {
    InitializeCandidate(initialize_candidate::InitializeCandidate),
    InitializePoll(initialize_poll::InitializePoll),
    Vote(vote::Vote),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for VotingDecoder {
    type InstructionType = VotingInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            VotingInstruction::InitializeCandidate => initialize_candidate::InitializeCandidate,
            VotingInstruction::InitializePoll => initialize_poll::InitializePoll,
            VotingInstruction::Vote => vote::Vote,
        )
    }
}