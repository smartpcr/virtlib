// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageSubSystemToReplicationCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageSubSystemToReplicationCapabilities {

/// 
    #[serde(rename = "ReplicationCapabilities")]
    pub replication_capabilities: Option<MSFT_ReplicationCapabilities>,

/// 
    #[serde(rename = "StorageSubSystem")]
    pub storage_sub_system: Option<MSFT_StorageSubSystem>,
}

impl MSFT_StorageSubSystemToReplicationCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            replication_capabilities: None,
            storage_sub_system: None,
        }
    }


    /// Sets the value of ReplicationCapabilities
    pub fn set_replication_capabilities(&mut self, value: MSFT_ReplicationCapabilities) {
        self.replication_capabilities = Some(value);
    }

    /// Gets the value of ReplicationCapabilities
    pub fn get_replication_capabilities(&self) -> Option<&MSFT_ReplicationCapabilities> {
        self.replication_capabilities.as_ref()
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

