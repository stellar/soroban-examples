use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "coinutils")]
#[command(about = "Privacy pool coin utilities")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a new coin
    Generate {
        /// Pool scope for the coin
        scope: String,
        /// Output file path
        #[arg(short, long, default_value = "coin.json")]
        output: String,
    },
    /// Withdraw a coin
    Withdraw {
        /// Coin file path
        coin_file: String,
        /// State file path
        state_file: String,
        /// Association set file path (optional)
        association_file: Option<String>,
        /// Output file path
        #[arg(short, long, default_value = "withdrawal.json")]
        output: String,
    },
    /// Update association set
    UpdateAssociation {
        /// Association set file path
        association_file: String,
        /// Label to add
        label: String,
    },
}

impl Cli {
    /// Parse command line arguments
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }

    /// Print usage information
    pub fn print_usage() {
        println!("Usage:");
        println!("  coinutils generate [scope] [output_file]  - Generate a new coin");
        println!("  coinutils withdraw <coin_file> <state_file> [association_set_file] [output_file]  - Withdraw a coin");
        println!("  coinutils updateAssociation <association_set_file> <label>  - Add label to association set");
        println!();
        println!("Examples:");
        println!("  coinutils generate my_pool_scope coin.json");
        println!("  coinutils withdraw coin.json state.json association.json withdrawal.json");
        println!("  coinutils updateAssociation association.json \"1234567890...\"");
        println!();
        println!("State file format:");
        println!("  {{");
        println!("    \"commitments\": [\"commitment1\", \"commitment2\", ...],");
        println!("    \"scope\": \"pool_scope\"");
        println!("  }}");
        println!();
        println!("Association set file format:");
        println!("  {{");
        println!("    \"labels\": [\"label1\", \"label2\", \"label3\", \"label4\"],");
        println!("    \"scope\": \"pool_scope\",");
        println!("    \"root\": \"merkle_tree_root\"");
        println!("  }}");
    }
}
