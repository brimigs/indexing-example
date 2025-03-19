 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

pub use super::VotingDecoder; 
pub mod candidate_account; 
pub mod poll_account; 

pub enum VotingAccount { 
        CandidateAccount(candidate_account::CandidateAccount), 
        PollAccount(poll_account::PollAccount), 
}


impl<'a> AccountDecoder<'a> for VotingDecoder { 
    type AccountType = VotingAccount;
     fn decode_account( &self, account: &solana_sdk::account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = candidate_account::CandidateAccount::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: VotingAccount::CandidateAccount(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = poll_account::PollAccount::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: VotingAccount::PollAccount(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}