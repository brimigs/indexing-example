

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xd26b76ccff61701a")]
pub struct InitializeCandidate{
    pub poll_id: u64,
    pub candidate: String,
}

pub struct InitializeCandidateInstructionAccounts {
    pub signer: solana_sdk::pubkey::Pubkey,
    pub poll_account: solana_sdk::pubkey::Pubkey,
    pub candidate_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeCandidate {
    type ArrangedAccounts = InitializeCandidateInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            signer,
            poll_account,
            candidate_account,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(InitializeCandidateInstructionAccounts {
            signer: signer.pubkey,
            poll_account: poll_account.pubkey,
            candidate_account: candidate_account.pubkey,
            system_program: system_program.pubkey,
        })
    }
}