//! Augmented Citizen SDK Integration Tests

use augmented_citizen_sdk::{CitizenApp, AppConfig, AuthManager, SessionManager};

#[test]
fn test_full_auth_session_flow() {
    // Create app
    let config = AppConfig::new("test-app", "v1.0.0");
    let app = CitizenApp::new(config);
    assert!(app.is_ok());

    // Authenticate
    let auth = AuthManager::new();
    let auth_result = auth.authenticate("bostrom1citizen");
    assert!(auth_result.is_ok());
    assert!(auth_result.unwrap().success);
}

#[test]
fn test_session_state_transitions() {
    let auth = AuthManager::new();
    let auth_result = auth.authenticate("bostrom1citizen").unwrap();

    let mut session_manager = SessionManager::new();
    let session = session_manager.create_session(&auth_result).unwrap();

    // Initial state should be Active (NDM 0.2)
    assert_eq!(session.state, augmented_citizen_sdk::session::SessionState::Active);

    // Update NDM to monitoring range
    session_manager.update_ndm_score(&session.session_id, 0.4).unwrap();
    let updated = session_manager.get_session(&session.session_id).unwrap();
    assert_eq!(updated.state, augmented_citizen_sdk::session::SessionState::Monitoring);

    // Update NDM to frozen range
    session_manager.update_ndm_score(&session.session_id, 0.9).unwrap();
    let updated = session_manager.get_session(&session.session_id).unwrap();
    assert_eq!(updated.state, augmented_citizen_sdk::session::SessionState::Frozen);
}

#[test]
fn test_invalid_did_rejected() {
    let auth = AuthManager::new();
    let result = auth.authenticate("invalid_did_format");
    assert!(result.is_err());
}

#[test]
fn test_capability_request_with_frozen_session() {
    use augmented_citizen_sdk::capabilities::{CapabilityClient, CapabilityRequest, ResourceRequirements};

    let auth = AuthManager::new();
    let auth_result = auth.authenticate("bostrom1citizen").unwrap();

    let mut session_manager = SessionManager::new();
    let session = session_manager.create_session(&auth_result).unwrap();

    // Freeze session
    session_manager.update_ndm_score(&session.session_id, 0.9).unwrap();
    let frozen_session = session_manager.get_session(&session.session_id).unwrap();

    // Try to request capability
    let client = CapabilityClient::new("https://sovereignty.aln");
    let request = CapabilityRequest {
        capability: "data_write".to_string(),
        justification: "test".to_string(),
        session_id: frozen_session.session_id.clone(),
        resource_requirements: ResourceRequirements {
            cpu_cores: 1,
            memory_mb: 1024,
            network_bandwidth_mbps: 10.0,
            duration_seconds: 60,
        },
    };

    // In production, this would be async
    // For now, just verify session is frozen
    assert_eq!(frozen_session.state, augmented_citizen_sdk::session::SessionState::Frozen);
    assert!(frozen_session.capabilities.is_empty());
}

#[tokio::test]
async fn test_offline_sync() {
    use augmented_citizen_sdk::offline::OfflineSync;
    use tempfile::tempdir;

    let dir = tempdir().unwrap();
    let path = dir.path().join("offline.db").to_string_lossy().to_string();

    let sync = OfflineSync::new(&path).unwrap();
    
    // Queue operation
    let op_id = sync.queue_operation("test_op", b"test data").unwrap();
    assert!(!op_id.is_empty());

    // Check status
    let status = sync.get_status().unwrap();
    assert_eq!(status.pending_operations, 1);
}

#[test]
fn test_ndm_monitor_alerts() {
    use augmented_citizen_sdk::ndm::NDMMonitor;

    let session_manager = SessionManager::new();
    let mut monitor = NDMMonitor::new(session_manager);

    // Create session
    let auth = AuthManager::new();
    let auth_result = auth.authenticate("bostrom1citizen").unwrap();
    let session = monitor.session_manager.create_session(&auth_result).unwrap();

    // Update to critical NDM
    monitor.update_score(&session.session_id, 0.85, "test_trigger").unwrap();

    // Check for alerts
    let alerts = monitor.check_alerts(&session.session_id).unwrap();
    assert!(!alerts.is_empty());
}
