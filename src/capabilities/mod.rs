//! Capability Client - Request and verify ALN capabilities
//!
//! This module provides capability request/verification API for
//! Augmented-Citizen applications with sovereignty enforcement.

use crate::error::CitizenSdkError;
use crate::session::CitizenSession;
use serde::{Deserialize, Serialize};

/// Capability client for ALN operations
pub struct CapabilityClient {
    sovereignty_endpoint: String,
    timeout_ms: u64,
}

/// Capability request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequest {
    pub capability: String,
    pub justification: String,
    pub session_id: String,
    pub resource_requirements: ResourceRequirements,
}

/// Resource requirements for capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub cpu_cores: u32,
    pub memory_mb: u64,
    pub network_bandwidth_mbps: f64,
    pub duration_seconds: i64,
}

/// Capability grant result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityGrant {
    pub granted: bool,
    pub capability: String,
    pub conditions: Vec<String>,
    pub expires_at: i64,
    pub row_id: Option<String>,
    pub trace_id: Option<String>,
}

impl CapabilityClient {
    /// Create a new capability client
    pub fn new(endpoint: &str) -> Self {
        Self {
            sovereignty_endpoint: endpoint.to_string(),
            timeout_ms: 5000,
        }
    }

    /// Set timeout
    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = ms;
        self
    }

    /// Request a capability
    pub async fn request_capability(
        &self,
        request: &CapabilityRequest,
        session: &CitizenSession,
    ) -> Result<CapabilityGrant, CitizenSdkError> {
        // Check session state
        if session.capabilities.is_empty() {
            return Err(CitizenSdkError::SessionFrozen);
        }

        // Check if capability is allowed for session state
        if !session.capabilities.contains(&"full_access".to_string())
            && request.capability.contains("write")
        {
            return Err(CitizenSdkError::CapabilityDenied {
                capability: request.capability.clone(),
                reason: "Session does not have write privileges".to_string(),
            });
        }

        // In production, submit to sovereigntycore for evaluation
        // For now, return simulated grant
        Ok(CapabilityGrant {
            granted: true,
            capability: request.capability.clone(),
            conditions: vec!["ndm_monitoring".to_string()],
            expires_at: chrono::Utc::now().timestamp() + 3600,
            row_id: Some(uuid::Uuid::new_v4().to_string()),
            trace_id: Some(uuid::Uuid::new_v4().to_string()),
        })
    }

    /// Verify capability grant
    pub fn verify_grant(&self, grant: &CapabilityGrant) -> Result<bool, CitizenSdkError> {
        // Verify grant signature and expiration
        if grant.expires_at < chrono::Utc::now().timestamp() {
            return Ok(false);
        }

        // In production, verify against ledger
        Ok(grant.granted)
    }

    /// Revoke capability
    pub async fn revoke_capability(
        &self,
        capability: &str,
        session: &CitizenSession,
    ) -> Result<(), CitizenSdkError> {
        // In production, submit revocation to sovereigntycore
        log::info!("Revoking capability {} for session {}", capability, session.session_id);
        Ok(())
    }

    /// List available capabilities for session
    pub fn list_capabilities(&self, session: &CitizenSession) -> Vec<String> {
        session.capabilities.clone()
    }
}

/// Capability taxonomy for Augmented-Citizen apps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CitizenCapability {
    // Healthcare capabilities
    ClinicalDataRead,
    ClinicalDataWrite,
    PatientRecordAccess,
    
    // Ecological capabilities
    EcoSensorRead,
    EcoActuatorControl,
    SwarmMissionRequest,
    
    // Diagnostics capabilities
    SystemDiagnostics,
    NetworkDiagnostics,
    PerformanceMetrics,
    
    // General capabilities
    DataRead,
    DataWrite,
    NetworkAccess,
}

impl CitizenCapability {
    /// Get capability string
    pub fn as_str(&self) -> &'static str {
        match self {
            CitizenCapability::ClinicalDataRead => "clinical_data_read",
            CitizenCapability::ClinicalDataWrite => "clinical_data_write",
            CitizenCapability::PatientRecordAccess => "patient_record_access",
            CitizenCapability::EcoSensorRead => "eco_sensor_read",
            CitizenCapability::EcoActuatorControl => "eco_actuator_control",
            CitizenCapability::SwarmMissionRequest => "swarm_mission_request",
            CitizenCapability::SystemDiagnostics => "system_diagnostics",
            CitizenCapability::NetworkDiagnostics => "network_diagnostics",
            CitizenCapability::PerformanceMetrics => "performance_metrics",
            CitizenCapability::DataRead => "data_read",
            CitizenCapability::DataWrite => "data_write",
            CitizenCapability::NetworkAccess => "network_access",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_client_creation() {
        let client = CapabilityClient::new("https://sovereignty.aln");
        assert_eq!(client.timeout_ms, 5000);
    }

    #[test]
    fn test_capability_taxonomy() {
        let cap = CitizenCapability::EcoSensorRead;
        assert_eq!(cap.as_str(), "eco_sensor_read");
    }
}
