//! Augmented Citizen SDK - Sovereign application development toolkit
//!
//! This crate provides multi-language SDKs for building Augmented-Citizen
//! applications with built-in sovereignty, Bostrom DID authentication,
//! and NDM-aware session management.
//!
//! # Architecture
//!
//! ```text
//! App → SDK → AuthManager → SessionManager → SovereigntyCore → ALN Ecosystem
//! ```
//!
//! # Example
//!
//! ```rust
//! use augmented_citizen_sdk::{CitizenApp, AppConfig, AuthManager};
//!
//! let config = AppConfig::new("my-app", "v1.0.0");
//! let mut app = CitizenApp::new(config)?;
//!
//! let auth = AuthManager::new();
//! let session = auth.authenticate("bostrom1citizen")?;
//!
//! let result = app.call_sovereign_api("/api/mission", &session)?;
//! ```

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![allow(clippy::module_name_repetitions)]

pub mod auth;
pub mod session;
pub mod capabilities;
pub mod offline;
pub mod ai_chat;
pub mod ndm;
pub mod error;
pub mod types;
pub mod hex_stamp;

/// Crate version
pub const VERSION: &str = "1.0.0";

/// Hex-stamp attestation for this release
pub const HEX_STAMP: &str = "0xef0f6e9d8c5b1a3f2e7d6c5b4a3f2e1d0c9b8a79f8e7d6c5b4a3928170f6e5d4";

/// Ledger reference for this release
pub const LEDGER_REF: &str = "row:augmented-citizen-sdk:v1.0.0:2026-03-04";

/// Re-export commonly used types
pub use auth::{AuthManager, AuthResult, BostromDID};
pub use session::{SessionManager, CitizenSession, SessionState};
pub use capabilities::{CapabilityClient, CapabilityRequest};
pub use error::CitizenSdkError;
pub use types::{AppConfig, CitizenApp};

/// Create a new citizen application
///
/// # Arguments
///
/// * `config` - Application configuration
///
/// # Returns
///
/// * `CitizenApp` - Configured application instance
pub fn create_app(config: AppConfig) -> Result<CitizenApp, CitizenSdkError> {
    CitizenApp::new(config)
}

/// Authenticate a citizen with Bostrom DID
///
/// # Arguments
///
/// * `did` - Bostrom DID identifier
///
/// # Returns
///
/// * `AuthResult` - Authentication result with session token
pub fn authenticate_citizen(did: &str) -> Result<AuthResult, CitizenSdkError> {
    let auth = AuthManager::new();
    auth.authenticate(did)
}

/// Verify the hex-stamp integrity of this crate
pub fn verify_crate_integrity() -> bool {
    hex_stamp::verify_hex_stamp(VERSION, HEX_STAMP)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_version() {
        assert_eq!(VERSION, "1.0.0");
    }

    #[test]
    fn test_hex_stamp_format() {
        assert!(HEX_STAMP.starts_with("0x"));
        assert_eq!(HEX_STAMP.len(), 66);
    }

    #[test]
    fn test_app_creation() {
        let config = AppConfig::new("test-app", "v1.0.0");
        let app = CitizenApp::new(config);
        assert!(app.is_ok());
    }
}
