//! Offline Sync - Offline-first data synchronization
//!
//! This module enables offline operation with later ledger anchoring
//! for Augmented-Citizen applications.

use crate::error::CitizenSdkError;
use serde::{Deserialize, Serialize};
use sled::Db;
use uuid::Uuid;
use chrono::Utc;

/// Offline sync manager
pub struct OfflineSync {
    db: Db,
    sync_queue_key: &'static [u8],
    anchor_queue_key: &'static [u8],
}

/// Sync operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncOperation {
    pub operation_id: String,
    pub operation_type: String,
    pub  Vec<u8>,
    pub created_at: i64,
    pub synced: bool,
    pub anchor_id: Option<String>,
}

/// Sync status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    pub pending_operations: usize,
    pub pending_anchors: usize,
    pub last_sync: Option<i64>,
    pub is_online: bool,
}

impl OfflineSync {
    /// Create a new offline sync manager
    pub fn new(db_path: &str) -> Result<Self, CitizenSdkError> {
        let db = sled::open(db_path)?;
        Ok(Self {
            db,
            sync_queue_key: b"sync_queue",
            anchor_queue_key: b"anchor_queue",
        })
    }

    /// Queue operation for sync
    pub fn queue_operation(&self, op_type: &str,  &[u8]) -> Result<String, CitizenSdkError> {
        let operation_id = Uuid::new_v4().to_string();
        
        let op = SyncOperation {
            operation_id: operation_id.clone(),
            operation_type: op_type.to_string(),
            data: data.to_vec(),
            created_at: Utc::now().timestamp(),
            synced: false,
            anchor_id: None,
        };

        // Add to queue
        let mut queue = self.get_sync_queue()?;
        queue.push(op);
        self.save_sync_queue(&queue)?;

        Ok(operation_id)
    }

    /// Sync pending operations
    pub async fn sync_pending(&self) -> Result<usize, CitizenSdkError> {
        let mut queue = self.get_sync_queue()?;
        let mut synced_count = 0;

        for op in &mut queue {
            if !op.synced {
                // In production, submit to sovereigntycore
                op.synced = true;
                synced_count += 1;
            }
        }

        self.save_sync_queue(&queue)?;
        Ok(synced_count)
    }

    /// Get sync status
    pub fn get_status(&self) -> Result<SyncStatus, CitizenSdkError> {
        let queue = self.get_sync_queue()?;
        let pending = queue.iter().filter(|op| !op.synced).count();

        Ok(SyncStatus {
            pending_operations: pending,
            pending_anchors: 0, // Would query anchor queue
            last_sync: None,    // Would track last sync time
            is_online: true,    // Would check network status
        })
    }

    /// Get sync queue
    fn get_sync_queue(&self) -> Result<Vec<SyncOperation>, CitizenSdkError> {
        match self.db.get(self.sync_queue_key)? {
            Some(data) => Ok(bincode::deserialize(&data)?),
            None => Ok(Vec::new()),
        }
    }

    /// Save sync queue
    fn save_sync_queue(&self, queue: &[SyncOperation]) -> Result<(), CitizenSdkError> {
        let data = bincode::serialize(queue)?;
        self.db.insert(self.sync_queue_key, data)?;
        Ok(())
    }

    /// Clear synced operations
    pub fn clear_synced(&self) -> Result<usize, CitizenSdkError> {
        let mut queue = self.get_sync_queue()?;
        let initial_len = queue.len();
        
        queue.retain(|op| !op.synced);
        
        let cleared = initial_len - queue.len();
        self.save_sync_queue(&queue)?;
        
        Ok(cleared)
    }
}

/// Offline configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineConfig {
    pub db_path: String,
    pub auto_sync_interval_secs: u64,
    pub max_pending_operations: usize,
    pub encrypt_at_rest: bool,
}

impl Default for OfflineConfig {
    fn default() -> Self {
        Self {
            db_path: "/var/lib/aln/citizen_offline.db".to_string(),
            auto_sync_interval_secs: 300,
            max_pending_operations: 10000,
            encrypt_at_rest: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_offline_sync_creation() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("offline.db").to_string_lossy().to_string();
        
        let sync = OfflineSync::new(&path);
        assert!(sync.is_ok());
    }

    #[test]
    fn test_queue_operation() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("offline.db").to_string_lossy().to_string();
        
        let sync = OfflineSync::new(&path).unwrap();
        let op_id = sync.queue_operation("test_op", b"test data").unwrap();
        
        assert!(!op_id.is_empty());
    }
}
