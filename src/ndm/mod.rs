//! NDM Monitor - Real-time NDM score visibility
//!
//! This module provides real-time NDM monitoring for Augmented-Citizen
//! applications with automatic session state updates.

use crate::error::CitizenSdkError;
use crate::session::{SessionManager, SessionState};
use serde::{Deserialize, Serialize};
use chrono::Utc;

/// NDM monitor for citizen sessions
pub struct NDMMonitor {
    session_manager: SessionManager,
    update_interval_secs: u64,
}

/// NDM status report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NDMStatus {
    pub session_id: String,
    pub current_score: f64,
    pub state: SessionState,
    pub suspicion_triggers: Vec<String>,
    pub recent_changes: Vec<NDMChange>,
    pub last_updated: i64,
}

/// NDM change record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NDMChange {
    pub from_score: f64,
    pub to_score: f64,
    pub trigger: String,
    pub timestamp: i64,
}

/// NDM alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NDMAlert {
    pub alert_id: String,
    pub session_id: String,
    pub alert_type: String,
    pub message: String,
    pub severity: String,
    pub timestamp: i64,
}

impl NDMMonitor {
    /// Create a new NDM monitor
    pub fn new(session_manager: SessionManager) -> Self {
        Self {
            session_manager,
            update_interval_secs: 60,
        }
    }

    /// Set update interval
    pub fn with_update_interval(mut self, secs: u64) -> Self {
        self.update_interval_secs = secs;
        self
    }

    /// Get NDM status for session
    pub fn get_status(&self, session_id: &str) -> Result<NDMStatus, CitizenSdkError> {
        let session = self.session_manager.get_session(session_id)
            .ok_or(CitizenSdkError::SessionNotFound)?;

        Ok(NDMStatus {
            session_id: session.session_id.clone(),
            current_score: session.ndm_score,
            state: session.state.clone(),
            suspicion_triggers: vec![],
            recent_changes: vec![],
            last_updated: Utc::now().timestamp(),
        })
    }

    /// Update NDM score for session
    pub fn update_score(
        &mut self,
        session_id: &str,
        new_score: f64,
        trigger: &str,
    ) -> Result<NDMStatus, CitizenSdkError> {
        self.session_manager.update_ndm_score(session_id, new_score)?;
        self.get_status(session_id)
    }

    /// Check for NDM alerts
    pub fn check_alerts(&self, session_id: &str) -> Result<Vec<NDMAlert>, CitizenSdkError> {
        let status = self.get_status(session_id)?;
        let mut alerts = Vec::new();

        // Check for threshold crossings
        if status.current_score >= 0.8 && status.state != SessionState::Frozen {
            alerts.push(NDMAlert {
                alert_id: uuid::Uuid::new_v4().to_string(),
                session_id: session_id.to_string(),
                alert_type: "ndm_critical".to_string(),
                message: "NDM score critical - session may be frozen".to_string(),
                severity: "critical".to_string(),
                timestamp: Utc::now().timestamp(),
            });
        }

        if status.current_score >= 0.6 && status.current_score < 0.8 {
            alerts.push(NDMAlert {
                alert_id: uuid::Uuid::new_v4().to_string(),
                session_id: session_id.to_string(),
                alert_type: "ndm_warning".to_string(),
                message: "NDM score elevated - capabilities restricted".to_string(),
                severity: "warning".to_string(),
                timestamp: Utc::now().timestamp(),
            });
        }

        Ok(alerts)
    }

    /// Subscribe to NDM updates
    pub fn subscribe_updates(&self, session_id: &str) -> Result<tokio::sync::broadcast::Receiver<NDMStatus>, CitizenSdkError> {
        // In production, create broadcast channel for real-time updates
        let (_, rx) = tokio::sync::broadcast::channel(100);
        Ok(rx)
    }
}

/// NDM threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NDMThresholdConfig {
    pub normal_ceiling: f64,
    pub monitoring_ceiling: f64,
    pub observe_only_ceiling: f64,
    pub freeze_ceiling: f64,
}

impl Default for NDMThresholdConfig {
    fn default() -> Self {
        Self {
            normal_ceiling: 0.3,
            monitoring_ceiling: 0.6,
            observe_only_ceiling: 0.8,
            freeze_ceiling: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ndm_monitor_creation() {
        let session_manager = SessionManager::new();
        let monitor = NDMMonitor::new(session_manager);
        assert_eq!(monitor.update_interval_secs, 60);
    }

    #[test]
    fn test_ndm_alert_generation() {
        let session_manager = SessionManager::new();
        let mut monitor = NDMMonitor::new(session_manager);

        // Create test session
        let auth = crate::auth::AuthResult {
            success: true,
            did: "bostrom1citizen".to_string(),
            session_token: "token".to_string(),
            expires_at: Utc::now().timestamp() + 3600,
            ndm_score: 0.2,
            capabilities: vec!["full_access".to_string()],
        };

        let session = monitor.session_manager.create_session(&auth).unwrap();
        
        // Update to critical NDM score
        let status = monitor.update_score(&session.session_id, 0.85, "test_trigger").unwrap();
        
        assert!(status.current_score >= 0.8);
    }
}
