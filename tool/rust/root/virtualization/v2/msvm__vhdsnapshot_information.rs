// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VHDSnapshotInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VHDSnapshotInformation {

/// 
    #[serde(rename = "CreationTime")]
    pub creation_time: Option<String>,

/// 
    #[serde(rename = "FilePath")]
    pub file_path: Option<String>,

/// 
    #[serde(rename = "ParentPathsList")]
    pub parent_paths_list: Vec<String>,

/// 
    #[serde(rename = "ResilientChangeTrackingId")]
    pub resilient_change_tracking_id: Option<String>,

/// 
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: Option<String>,

/// 
    #[serde(rename = "SnapshotPath")]
    pub snapshot_path: Option<String>,
}

impl Msvm_VHDSnapshotInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            creation_time: None,
            file_path: None,
            parent_paths_list: Vec::new(),
            resilient_change_tracking_id: None,
            snapshot_id: None,
            snapshot_path: None,
        }
    }


    /// Sets the value of CreationTime
    pub fn set_creation_time(&mut self, value: String) {
        self.creation_time = Some(value);
    }

    /// Gets the value of CreationTime
    pub fn get_creation_time(&self) -> Option<&String> {
        self.creation_time.as_ref()
    }

    /// Sets the value of FilePath
    pub fn set_file_path(&mut self, value: String) {
        self.file_path = Some(value);
    }

    /// Gets the value of FilePath
    pub fn get_file_path(&self) -> Option<&String> {
        self.file_path.as_ref()
    }

    /// Sets the value of ParentPathsList
    pub fn set_parent_paths_list(&mut self, value: Vec<String>) {
        self.parent_paths_list = value;
    }

    /// Gets the value of ParentPathsList
    pub fn get_parent_paths_list(&self) -> &Vec<String> {
        &self.parent_paths_list
    }

    /// Sets the value of ResilientChangeTrackingId
    pub fn set_resilient_change_tracking_id(&mut self, value: String) {
        self.resilient_change_tracking_id = Some(value);
    }

    /// Gets the value of ResilientChangeTrackingId
    pub fn get_resilient_change_tracking_id(&self) -> Option<&String> {
        self.resilient_change_tracking_id.as_ref()
    }

    /// Sets the value of SnapshotId
    pub fn set_snapshot_id(&mut self, value: String) {
        self.snapshot_id = Some(value);
    }

    /// Gets the value of SnapshotId
    pub fn get_snapshot_id(&self) -> Option<&String> {
        self.snapshot_id.as_ref()
    }

    /// Sets the value of SnapshotPath
    pub fn set_snapshot_path(&mut self, value: String) {
        self.snapshot_path = Some(value);
    }

    /// Gets the value of SnapshotPath
    pub fn get_snapshot_path(&self) -> Option<&String> {
        self.snapshot_path.as_ref()
    }
}

