// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VHDSetInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VHDSetInformation {

/// 
    #[serde(rename = "AllPaths")]
    pub all_paths: Vec<String>,

/// 
    #[serde(rename = "Path")]
    pub path: Option<String>,

/// 
    #[serde(rename = "SnapshotIdList")]
    pub snapshot_id_list: Vec<String>,
}

impl Msvm_VHDSetInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            all_paths: Vec::new(),
            path: None,
            snapshot_id_list: Vec::new(),
        }
    }


    /// Sets the value of AllPaths
    pub fn set_all_paths(&mut self, value: Vec<String>) {
        self.all_paths = value;
    }

    /// Gets the value of AllPaths
    pub fn get_all_paths(&self) -> &Vec<String> {
        &self.all_paths
    }

    /// Sets the value of Path
    pub fn set_path(&mut self, value: String) {
        self.path = Some(value);
    }

    /// Gets the value of Path
    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// Sets the value of SnapshotIdList
    pub fn set_snapshot_id_list(&mut self, value: Vec<String>) {
        self.snapshot_id_list = value;
    }

    /// Gets the value of SnapshotIdList
    pub fn get_snapshot_id_list(&self) -> &Vec<String> {
        &self.snapshot_id_list
    }
}

