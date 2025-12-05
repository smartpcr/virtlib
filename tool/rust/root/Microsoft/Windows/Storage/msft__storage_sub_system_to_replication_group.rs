// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageSubSystemToReplicationGroup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageSubSystemToReplicationGroup {

/// 
    #[serde(rename = "ReplicationGroup")]
    pub replication_group: Option<MSFT_ReplicationGroup>,

/// 
    #[serde(rename = "StorageSubSystem")]
    pub storage_sub_system: Option<MSFT_StorageSubSystem>,
}

impl MSFT_StorageSubSystemToReplicationGroup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            replication_group: None,
            storage_sub_system: None,
        }
    }


    /// Sets the value of ReplicationGroup
    pub fn set_replication_group(&mut self, value: MSFT_ReplicationGroup) {
        self.replication_group = Some(value);
    }

    /// Gets the value of ReplicationGroup
    pub fn get_replication_group(&self) -> Option<&MSFT_ReplicationGroup> {
        self.replication_group.as_ref()
    }

    /// Sets the value of StorageSubSystem
    pub fn set_storage_sub_system(&mut self, value: MSFT_StorageSubSystem) {
        self.storage_sub_system = Some(value);
    }

    /// Gets the value of StorageSubSystem
    pub fn get_storage_sub_system(&self) -> Option<&MSFT_StorageSubSystem> {
        self.storage_sub_system.as_ref()
    }
}

