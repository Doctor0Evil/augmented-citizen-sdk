//! Session Management - NDM-aware session lifecycle
//!
//! This module manages citizen sessions with NDM-aware privilege
//! escalation and automatic suspension on NDM threshold breaches.

use crate::error::CitizenSdkError;
use crate::auth::AuthResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Session manager for citizen sessions
pub struct SessionManager {
    active_sessions: std::collections::HashMap<String, CitizenSession>,
    ndm_threshold_freeze: f64,
}

/// Citizen session state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SessionState {
    Active,
    Monitoring,
    ObserveOnly,
    Frozen,
    Expired,
}

/// Citizen session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitizenSession {
    pub session_id: String,
    pub did: String,
    pub state: SessionState,
    pub ndm_score: f64,
    pub capabilities: Vec<String>,
    pub created_at: i64,
    pub expires_at: i64,
    pub last_activity: i64,
}

/// Session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    pub timeout_secs: u64,
    pub ndm_freeze_threshold: f64,
    pub ndm_monitoring_threshold: f64,
    pub auto_refresh: bool,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            timeout_secs: 3600,
            ndm_freeze_threshold: 0.8,
            ndm_monitoring_threshold: 0.3,
            auto_refresh: false,
        }
    }
}

impl SessionManager {
    /// Create a new session manager
    pub fn new() -> Self {
        Self {
            active_sessions: std::collections::HashMap::new(),
            ndm_threshold_freeze: 0.8,
        }
    }

    /// Create session from auth result
    pub fn create_session(&mut self, auth: &AuthResult) -> Result<CitizenSession, CitizenSdkError> {
        let session_id = Uuid::new_v4().to_string();
        let now = Utc::now().timestamp();

        let state = self.determine_session_state(auth.ndm_score);

        let session = CitizenSession {
            session_id,
            did: auth.did.clone(),
            state,
            ndm_score: auth.ndm_score,
            capabilities: auth.capabilities.clone(),
            created_at: now,
            expires_at: auth.expires_at,
            last_activity: now,
        };

        self.active_sessions.insert(session.session_id.clone(), session.clone());

        Ok(session)
    }

    /// Determine session state from NDM score
    fn determine_session_state(&self, ndm_score: f64) -> SessionState {
        if ndm_score >= self.ndm_threshold_freeze {
            SessionState::Frozen
        } else if ndm_score >= 0.6 {
            SessionState::ObserveOnly
        } else if ndm_score >= 0.3 {
            SessionState::Monitoring
        } else {
            SessionState::Active
        }
    }

    /// Get active session
    pub fn get_session(&self, session_id: &str) -> Option<&CitizenSession> {
        self.active_sessions.get(session_id)
    }

    /// Update session NDM score
    pub fn update_ndm_score(&mut self, session_id: &str, new_score: f64) -> Result<(), CitizenSdkError> {
        let session = self.active_sessions.get_mut(session_id)
            .ok_or(CitizenSdkError::SessionNotFound)?;

        session.ndm_score = new_score;
        session.state = self.determine_session_state(new_score);
        session.capabilities = self.get_capabilities_for_state(&session.state);
        session.last_activity = Utc::now().timestamp();

        Ok(())
    }

    /// Get capabilities for session state
    fn get_capabilities_for_state(&self, state: &SessionState) -> Vec<String> {
        match state {
            SessionState::Active => vec!["full_access".to_string(), "write".to_string(), "execute".to_string()],
            SessionState::Monitoring => vec!["read".to_string(), "write".to_string()],
            SessionState::ObserveOnly => vec!["read".to_string()],
            SessionState::Frozen => vec![],
            SessionState::Expired => vec![],
        }
    }

    /// Expire session
    pub fn expire_session(&mut self, session_id: &str) -> Result<(), CitizenSdkError> {
        let session = self.active_sessions.get_mut(session_id)
            .ok_or(CitizenSdkError::SessionNotFound)?;

        session.state = SessionState::Expired;
        session.capabilities = vec![];

        Ok(())
    }

    /// Get all active sessions
    pub fn get_active_sessions(&self) -> Vec<&CitizenSession> {
        self.active_sessions
            .values()
            .filter(|s| s.state == SessionState::Active)
            .collect()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_manager_creation() {
        let manager = SessionManager::new();
        assert_eq!(manager.ndm_threshold_freeze, 0.8);
    }

    #[test]
    fn test_session_state_determination() {
        let manager = SessionManager::new();
        
        assert_eq!(manager.determine_session_state(0.2), SessionState::Active);
        assert_eq!(manager.determine_session_state(0.4), SessionState::Monitoring);
        assert_eq!(manager.determine_session_state(0.7), SessionState::ObserveOnly);
        assert_eq!(manager.determine_session_state(0.9), SessionState::Frozen);
    }

    #[test]
    fn test_session_creation() {
        let mut manager = SessionManager::new();
        let auth = AuthResult {
            success: true,
            did: "bostrom1citizen".to_string(),
            session_token: "token-123".to_string(),
            expires_at: Utc::now().timestamp() + 3600,
            ndm_score: 0.2,
            capabilities: vec!["full_access".to_string()],
        };

        let session = manager.create_session(&auth);
        assert!(session.is_ok());
        assert_eq!(session.unwrap().state, SessionState::Active);
    }
}
