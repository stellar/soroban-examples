use crate::{
    crypto::coin::generate_coin,
    error::Result,
    io::{FileManager, SerializationManager},
    merkle::association::AssociationManager,
    merkle::withdrawal::WithdrawalManager,
};
use log::{debug, info};
use soroban_sdk::Env;

/// Command handler for processing CLI commands
pub struct CommandHandler {
    file_manager: FileManager,
    serialization_manager: SerializationManager,
    withdrawal_manager: WithdrawalManager,
    association_manager: AssociationManager,
}

impl CommandHandler {
    pub fn new() -> Self {
        Self {
            file_manager: FileManager::new(),
            serialization_manager: SerializationManager::new(),
            withdrawal_manager: WithdrawalManager::new(),
            association_manager: AssociationManager::new(),
        }
    }

    /// Handle the generate command
    pub fn handle_generate(&self, scope: String, output: String) -> Result<()> {
        info!("Generating coin with scope: {}", scope);
        debug!("Output file: {}", output);

        let env = Env::default();
        env.cost_estimate().budget().reset_unlimited();

        let generated_coin = generate_coin(&env, scope.as_bytes());
        debug!(
            "Generated coin commitment: {}",
            generated_coin.commitment_hex
        );

        // Save coin data
        self.file_manager
            .write_coin_file(&generated_coin, &output)?;
        info!("Coin saved to: {}", output);

        println!("Generated coin:");
        println!("  Value: {}", generated_coin.coin.value);
        println!("  Nullifier: {}", generated_coin.coin.nullifier);
        println!("  Secret: {}", generated_coin.coin.secret);
        println!("  Label: {}", generated_coin.coin.label);
        println!("  Commitment: {}", generated_coin.commitment_hex);
        println!("  Saved to: {}", output);

        Ok(())
    }

    /// Handle the withdraw command
    pub fn handle_withdraw(
        &self,
        coin_file: String,
        state_file: String,
        association_file: Option<String>,
        output: String,
    ) -> Result<()> {
        info!("Processing withdrawal for coin: {}", coin_file);
        debug!("State file: {}", state_file);
        debug!("Association file: {:?}", association_file);
        debug!("Output file: {}", output);

        let env = Env::default();
        env.cost_estimate().budget().reset_unlimited();

        // Read existing coin
        let existing_coin = self.file_manager.read_coin_file(&coin_file)?;

        // Read state file
        let state_data = self.file_manager.read_state_file(&state_file)?;

        // Read association set file if provided
        let association_set_data = if let Some(assoc_file) = association_file {
            Some(self.file_manager.read_association_file(&assoc_file)?)
        } else {
            None
        };

        // Generate withdrawal
        let snark_input = self.withdrawal_manager.withdraw_coin(
            &env,
            &existing_coin.coin,
            &state_data,
            association_set_data.as_ref(),
        )?;

        // Save withdrawal data
        let withdrawal_json = self
            .serialization_manager
            .serialize_snark_input(&snark_input)?;
        std::fs::write(&output, withdrawal_json)?;
        info!("Withdrawal data saved to: {}", output);

        println!("Withdrawal created:");
        println!("  Withdrawn value: {}", snark_input.withdrawn_value);
        println!("  State root: {}", snark_input.state_root);
        println!("  Association root: {}", snark_input.association_root);
        println!("  Commitment index: {}", snark_input.state_index);
        println!("  Snark input saved to: {}", output);

        Ok(())
    }

    /// Handle the updateAssociation command
    pub fn handle_update_association(&self, association_file: String, label: String) -> Result<()> {
        info!("Updating association set: {}", association_file);
        debug!("Adding label: {}", label);

        let env = Env::default();
        env.cost_estimate().budget().reset_unlimited();

        self.association_manager
            .update_association_set(&env, &association_file, &label)?;
        info!("Association set updated successfully");

        println!("Association set updated successfully");
        Ok(())
    }
}

impl Default for CommandHandler {
    fn default() -> Self {
        Self::new()
    }
}
