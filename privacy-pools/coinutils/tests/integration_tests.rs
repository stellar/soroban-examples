use coinutils::{
    cli::CommandHandler,
    crypto::coin::generate_coin,
    io::FileManager,
    types::{StateFile, AssociationSetFile},
    error::Result,
};
use soroban_sdk::Env;
use tempfile::TempDir;

#[tokio::test]
async fn test_full_coin_lifecycle() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let env = Env::default();
    env.cost_estimate().budget().reset_unlimited();
    
    let file_manager = FileManager::new();
    let command_handler = CommandHandler::new();
    
    // Step 1: Generate a coin
    let scope = b"test_scope";
    let generated_coin = generate_coin(&env, scope);
    
    let coin_file = temp_dir.path().join("coin.json");
    file_manager.write_coin_file(&generated_coin, coin_file.to_str().unwrap())?;
    
    // Step 2: Create a state file with the coin's commitment
    let state_file = StateFile {
        commitments: vec![generated_coin.coin.commitment.clone()],
        scope: "test_scope".to_string(),
        association_set: None,
    };
    
    let state_file_path = temp_dir.path().join("state.json");
    file_manager.write_state_file(&state_file, state_file_path.to_str().unwrap())?;
    
    // Step 3: Create an association set
    let association_file = AssociationSetFile {
        labels: vec![generated_coin.coin.label.clone()],
        scope: "test_scope".to_string(),
        root: None,
    };
    
    let association_file_path = temp_dir.path().join("association.json");
    file_manager.write_association_file(&association_file, association_file_path.to_str().unwrap())?;
    
    // Step 4: Withdraw the coin
    let withdrawal_file = temp_dir.path().join("withdrawal.json");
    command_handler.handle_withdraw(
        coin_file.to_str().unwrap().to_string(),
        state_file_path.to_str().unwrap().to_string(),
        Some(association_file_path.to_str().unwrap().to_string()),
        withdrawal_file.to_str().unwrap().to_string(),
    )?;
    
    // Verify the withdrawal file was created
    assert!(withdrawal_file.exists());
    
    Ok(())
}

#[tokio::test]
async fn test_association_set_management() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let env = Env::default();
    env.cost_estimate().budget().reset_unlimited();
    
    let command_handler = CommandHandler::new();
    let association_file = temp_dir.path().join("association.json");
    
    // Add first label
    command_handler.handle_update_association(
        association_file.to_str().unwrap().to_string(),
        "123456789".to_string(),
    )?;
    
    // Add second label
    command_handler.handle_update_association(
        association_file.to_str().unwrap().to_string(),
        "987654321".to_string(),
    )?;
    
    // Verify the association file was created and has both labels
    let file_manager = FileManager::new();
    let association = file_manager.read_association_file(association_file.to_str().unwrap())?;
    
    assert_eq!(association.labels.len(), 2);
    assert!(association.labels.contains(&"123456789".to_string()));
    assert!(association.labels.contains(&"987654321".to_string()));
    assert!(association.root.is_some());
    
    Ok(())
}

#[tokio::test]
async fn test_coin_generation() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let env = Env::default();
    env.cost_estimate().budget().reset_unlimited();
    
    let command_handler = CommandHandler::new();
    let output_file = temp_dir.path().join("coin.json");
    
    // Generate a coin
    command_handler.handle_generate(
        "test_scope".to_string(),
        output_file.to_str().unwrap().to_string(),
    )?;
    
    // Verify the coin file was created
    assert!(output_file.exists());
    
    // Read and verify the coin data
    let file_manager = FileManager::new();
    let coin = file_manager.read_coin_file(output_file.to_str().unwrap())?;
    
    assert!(!coin.coin.value.is_empty());
    assert!(!coin.coin.nullifier.is_empty());
    assert!(!coin.coin.secret.is_empty());
    assert!(!coin.coin.label.is_empty());
    assert!(!coin.coin.commitment.is_empty());
    assert!(coin.commitment_hex.starts_with("0x"));
    
    Ok(())
}
