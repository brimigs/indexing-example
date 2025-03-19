// Import necessary dependencies and modules.
use {
    async_trait::async_trait,
    carbon_core::{
        // Common error and result types used throughout Carbon.
        error::CarbonResult,
        // Types used for decoding instructions.
        instruction::{DecodedInstruction, InstructionMetadata, NestedInstruction},
        // Metrics collection type (can be used for reporting performance).
        metrics::MetricsCollection,
        // Pipeline builder and processor traits.
        pipeline::Pipeline,
        processor::Processor,
        // Account-related types used to decode account updates.
        account::{AccountMetadata, DecodedAccount},
    },
    // The Yellowstone gRPC datasource for streaming blockchain data.
    carbon_yellowstone_grpc_datasource::YellowstoneGrpcGeyserClient,
    // Standard library imports.
    std::{
        collections::{HashMap, HashSet},
        env,
        sync::Arc,
    },
    // Tokio async runtime's RwLock.
    tokio::sync::RwLock,
    // gRPC protocol definitions for filters and commitment levels.
    yellowstone_grpc_proto::geyser::{
        CommitmentLevel, SubscribeRequestFilterAccounts, SubscribeRequestFilterTransactions,
    },
    // // Import your custom voting instruction decoder and types.
    // carbon_voting_decoder::instructions::VotingInstruction,
    // // Import your custom voting account decoder and account type.
    // carbon_voting_decoder::accounts::VotingAccount,
    carbon_voting_decoder::{
        InstructionVotingDecoder,
        AccountVotingDecoder,
        VotingInstruction,
        VotingAccount,
    },
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    // Initialize logging and load environment variables from a .env file.
    env_logger::init();
    dotenv::dotenv().ok();

    // Retrieve the Voting Program ID from environment variables.
    // This is used to filter transactions/accounts related to your voting program.
    let voting_program_id = env::var("VOTING_PROGRAM_ID")
        .expect("VOTING_PROGRAM_ID must be set in your environment variables");

    // Set up account filters to capture account updates related to your voting program.
    let mut account_filters: HashMap<String, SubscribeRequestFilterAccounts> = HashMap::new();
    account_filters.insert(
        "voting_account_filter".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![], // No specific account addresses provided.
            owner: vec![voting_program_id.clone()], // Filter by the owner (voting program).
            filters: vec![], // No additional filters.
            nonempty_txn_signature: None,
        },
    );

    // Set up transaction filters to capture transactions involving your voting program.
    let transaction_filter = SubscribeRequestFilterTransactions {
        vote: Some(false), // Do not filter based on vote transactions.
        failed: Some(false), // Exclude failed transactions.
        account_include: vec![], // No additional inclusion filters.
        account_exclude: vec![], // No exclusions.
        account_required: vec![voting_program_id.clone()], // Only include transactions related to the voting program.
        signature: None,
    };

    let mut transaction_filters: HashMap<String, SubscribeRequestFilterTransactions> =
        HashMap::new();
    transaction_filters.insert("voting_transaction_filter".to_string(), transaction_filter);

    // Create the Yellowstone gRPC datasource using the filters defined above.
    // This datasource connects to the blockchain RPC endpoint and streams the relevant data.
    let yellowstone_grpc = YellowstoneGrpcGeyserClient::new(
        env::var("GEYSER_URL").unwrap_or_default(), // RPC URL for the datasource.
        env::var("X_TOKEN").ok(), // Optional authentication token.
        Some(CommitmentLevel::Confirmed), // Use confirmed commitment level.
        account_filters,
        transaction_filters,
        Arc::new(RwLock::new(HashSet::new())),
    );

    // Build the Carbon pipeline:
    // - Provide the datasource to stream data.
    // - Register the instruction processor with your custom VotingDecoder.
    // - Register the account processor with your custom VotingAccountDecoder.
    // - Set the shutdown strategy (here, immediate shutdown when done).
    Pipeline::builder()
        .datasource(yellowstone_grpc)
        .instruction(InstructionVotingDecoder, VotingInstructionProcessor)
        .account(AccountVotingDecoder, VotingAccountProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

/// A processor for handling decoded voting instructions.
/// This implementation logs the transaction signature and instruction data.
pub struct VotingInstructionProcessor;

#[async_trait]
impl Processor for VotingInstructionProcessor {
    // The InputType is a tuple consisting of:
    // - InstructionMetadata: Metadata about the transaction.
    // - DecodedInstruction<VotingInstruction>: The decoded voting instruction.
    // - Vec<NestedInstruction>: Any nested instructions (if applicable).
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<VotingInstruction>,
        Vec<NestedInstruction>,
    );

    async fn process(
        &mut self,
        (metadata, instruction, _nested_instructions): Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let signature = metadata.transaction_metadata.signature;

        let signature = format!(
            "{}...{}",
            &signature.to_string()[..4],
            &signature.to_string()[signature.to_string().len() - 4..signature.to_string().len()]
        );

        println!(
            "instruction processed ({}) {:?}",
            signature, instruction.data
        );

        Ok(())
    }
}

/// A processor for handling decoded voting account updates.
/// This implementation logs a shortened version of the account's public key and its updated data.
pub struct VotingAccountProcessor;

#[async_trait]
impl Processor for VotingAccountProcessor {
    // The InputType is a tuple consisting of:
    // - AccountMetadata: Metadata for the account (including the public key).
    // - DecodedAccount<VotingAccount>: The decoded voting account data.
    type InputType = (AccountMetadata, DecodedAccount<VotingAccount>);

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let account = data.1;

        let pubkey_str = format!(
            "{}...{}",
            &data.0.pubkey.to_string()[..4],
            &data.0.pubkey.to_string()[4..]
        );

        // fn max_total_chars(s: &str, max: usize) -> String {
        //     if s.len() > max {
        //         format!("{}...", &s[..max])
        //     } else {
        //         s.to_string()
        //     }
        // }

        // Log the account update.
        println!(
            "Voting account updated ({})",
            pubkey_str,
        );
        Ok(())
    }
}