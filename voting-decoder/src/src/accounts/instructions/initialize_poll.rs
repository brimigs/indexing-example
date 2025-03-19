

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xc11663c512217375")]
pub struct InitializePoll{
    pub poll_id: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub name: String,
    pub description: String,
}

pub struct InitializePollInstructionAccounts {
    pub signer: solana_sdk::pubkey::Pubkey,
    pub poll_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePoll {
    type ArrangedAccounts = InitializePollInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            signer,
            poll_account,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(InitializePollInstructionAccounts {
            signer: signer.pubkey,
            poll_account: poll_account.pubkey,
            system_program: system_program.pubkey,
        })
    }
}