

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe36e9b17887eac19")]
pub struct Vote{
    pub poll_id: u64,
    pub candidate: String,
}

pub struct VoteInstructionAccounts {
    pub signer: solana_sdk::pubkey::Pubkey,
    pub poll_account: solana_sdk::pubkey::Pubkey,
    pub candidate_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Vote {
    type ArrangedAccounts = VoteInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            signer,
            poll_account,
            candidate_account,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(VoteInstructionAccounts {
            signer: signer.pubkey,
            poll_account: poll_account.pubkey,
            candidate_account: candidate_account.pubkey,
        })
    }
}