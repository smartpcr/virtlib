// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_Synchronized struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_Synchronized {

/// 
    #[serde(rename = "CopyMethodology")]
    pub copy_methodology: Option<u16>,

/// 
    #[serde(rename = "CopyPriority")]
    pub copy_priority: Option<u16>,

/// 
    #[serde(rename = "CopyState")]
    pub copy_state: Option<u16>,

/// 
    #[serde(rename = "CopyType")]
    pub copy_type: Option<u16>,

/// 
    #[serde(rename = "PercentSynced")]
    pub percent_synced: Option<u16>,

/// 
    #[serde(rename = "ProgressStatus")]
    pub progress_status: Option<u16>,

/// 
    #[serde(rename = "RecoveryPointObjective")]
    pub recovery_point_objective: Option<u32>,

/// 
    #[serde(rename = "ReplicaType")]
    pub replica_type: Option<u16>,

/// 
    #[serde(rename = "RequestedCopyState")]
    pub requested_copy_state: Option<u16>,

/// 
    #[serde(rename = "SyncMaintained")]
    pub sync_maintained: Option<bool>,

/// 
    #[serde(rename = "SyncMode")]
    pub sync_mode: Option<u16>,

/// 
    #[serde(rename = "SyncState")]
    pub sync_state: Option<u16>,

/// 
    #[serde(rename = "SyncTime")]
    pub sync_time: Option<String>,

/// 
    #[serde(rename = "SyncType")]
    pub sync_type: Option<u16>,
}

impl MSFT_Synchronized {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            copy_methodology: None,
            copy_priority: None,
            copy_state: None,
            copy_type: None,
            percent_synced: None,
            progress_status: None,
            recovery_point_objective: None,
            replica_type: None,
            requested_copy_state: None,
            sync_maintained: None,
            sync_mode: None,
            sync_state: None,
            sync_time: None,
            sync_type: None,
        }
    }


    /// Sets the value of CopyMethodology
    pub fn set_copy_methodology(&mut self, value: u16) {
        self.copy_methodology = Some(value);
    }

    /// Gets the value of CopyMethodology
    pub fn get_copy_methodology(&self) -> Option<&u16> {
        self.copy_methodology.as_ref()
    }

    /// Sets the value of CopyPriority
    pub fn set_copy_priority(&mut self, value: u16) {
        self.copy_priority = Some(value);
    }

    /// Gets the value of CopyPriority
    pub fn get_copy_priority(&self) -> Option<&u16> {
        self.copy_priority.as_ref()
    }

    /// Sets the value of CopyState
    pub fn set_copy_state(&mut self, value: u16) {
        self.copy_state = Some(value);
    }

    /// Gets the value of CopyState
    pub fn get_copy_state(&self) -> Option<&u16> {
        self.copy_state.as_ref()
    }

    /// Sets the value of CopyType
    pub fn set_copy_type(&mut self, value: u16) {
        self.copy_type = Some(value);
    }

    /// Gets the value of CopyType
    pub fn get_copy_type(&self) -> Option<&u16> {
        self.copy_type.as_ref()
    }

    /// Sets the value of PercentSynced
    pub fn set_percent_synced(&mut self, value: u16) {
        self.percent_synced = Some(value);
    }

    /// Gets the value of PercentSynced
    pub fn get_percent_synced(&self) -> Option<&u16> {
        self.percent_synced.as_ref()
    }

    /// Sets the value of ProgressStatus
    pub fn set_progress_status(&mut self, value: u16) {
        self.progress_status = Some(value);
    }

    /// Gets the value of ProgressStatus
    pub fn get_progress_status(&self) -> Option<&u16> {
        self.progress_status.as_ref()
    }

    /// Sets the value of RecoveryPointObjective
    pub fn set_recovery_point_objective(&mut self, value: u32) {
        self.recovery_point_objective = Some(value);
    }

    /// Gets the value of RecoveryPointObjective
    pub fn get_recovery_point_objective(&self) -> Option<&u32> {
        self.recovery_point_objective.as_ref()
    }

    /// Sets the value of ReplicaType
    pub fn set_replica_type(&mut self, value: u16) {
        self.replica_type = Some(value);
    }

    /// Gets the value of ReplicaType
    pub fn get_replica_type(&self) -> Option<&u16> {
        self.replica_type.as_ref()
    }

    /// Sets the value of RequestedCopyState
    pub fn set_requested_copy_state(&mut self, value: u16) {
        self.requested_copy_state = Some(value);
    }

    /// Gets the value of RequestedCopyState
    pub fn get_requested_copy_state(&self) -> Option<&u16> {
        self.requested_copy_state.as_ref()
    }

    /// Sets the value of SyncMaintained
    pub fn set_sync_maintained(&mut self, value: bool) {
        self.sync_maintained = Some(value);
    }

    /// Gets the value of SyncMaintained
    pub fn get_sync_maintained(&self) -> Option<&bool> {
        self.sync_maintained.as_ref()
    }

    /// Sets the value of SyncMode
    pub fn set_sync_mode(&mut self, value: u16) {
        self.sync_mode = Some(value);
    }

    /// Gets the value of SyncMode
    pub fn get_sync_mode(&self) -> Option<&u16> {
        self.sync_mode.as_ref()
    }

    /// Sets the value of SyncState
    pub fn set_sync_state(&mut self, value: u16) {
        self.sync_state = Some(value);
    }

    /// Gets the value of SyncState
    pub fn get_sync_state(&self) -> Option<&u16> {
        self.sync_state.as_ref()
    }

    /// Sets the value of SyncTime
    pub fn set_sync_time(&mut self, value: String) {
        self.sync_time = Some(value);
    }

    /// Gets the value of SyncTime
    pub fn get_sync_time(&self) -> Option<&String> {
        self.sync_time.as_ref()
    }

    /// Sets the value of SyncType
    pub fn set_sync_type(&mut self, value: u16) {
        self.sync_type = Some(value);
    }

    /// Gets the value of SyncType
    pub fn get_sync_type(&self) -> Option<&u16> {
        self.sync_type.as_ref()
    }
}

