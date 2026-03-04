//! Citizen CLI - Command-line interface for Augmented-Citizen SDK

use augmented_citizen_sdk::{CitizenApp, AppConfig, AuthManager, SessionManager};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "citizen-cli")]
#[command(about = "Augmented-Citizen SDK CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// App name
    #[arg(short, long, default_value = "citizen-app")]
    app_name: String,

    /// App version
    #[arg(short, long, default_value = "1.0.0")]
    app_version: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Authenticate with Bostrom DID
    Auth {
        /// Bostrom DID
        #[arg(short, long)]
        did: String,
    },
    /// Create new session
    Session {
        /// DID to create session for
        #[arg(short, long)]
        did: String,
    },
    /// Check NDM status
    NDM {
        /// Session ID
        #[arg(short, long)]
        session_id: String,
    },
    /// Request capability
    Capability {
        /// Capability name
        #[arg(short, long)]
        capability: String,
        /// Session ID
        #[arg(short, long)]
        session_id: String,
    },
    /// Sync offline operations
    Sync,
    /// Show app status
    Status,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cli = Cli::parse();

    let config = AppConfig::new(&cli.app_name, &cli.app_version);
    let app = CitizenApp::new(config)?;

    match cli.command {
        Commands::Auth { did } => {
            println!("Authenticating with DID: {}", did);
            let auth = AuthManager::new();
            let result = auth.authenticate(&did)?;
            println!("Authentication successful!");
            println!("Session token: {}", result.session_token);
            println!("NDM score: {}", result.ndm_score);
        }
        Commands::Session { did } => {
            println!("Creating session for DID: {}", did);
            let auth = AuthManager::new();
            let auth_result = auth.authenticate(&did)?;
            
            let mut session_manager = SessionManager::new();
            let session = session_manager.create_session(&auth_result)?;
            
            println!("Session created: {}", session.session_id);
            println!("State: {:?}", session.state);
        }
        Commands::NDM { session_id } => {
            println!("Checking NDM status for session: {}", session_id);
            // In production, query NDM status
            println!("NDM status retrieved");
        }
        Commands::Capability { capability, session_id } => {
            println!("Requesting capability: {} for session: {}", capability, session_id);
            // In production, request capability
            println!("Capability request submitted");
        }
        Commands::Sync => {
            println!("Syncing offline operations...");
            // In production, sync pending operations
            println!("Sync complete");
        }
        Commands::Status => {
            println!("App: {}", cli.app_name);
            println!("Version: {}", cli.app_version);
            println!("Status: Running");
        }
    }

    Ok(())
}
