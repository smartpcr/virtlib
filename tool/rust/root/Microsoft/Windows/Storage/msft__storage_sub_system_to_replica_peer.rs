// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageSubSystemToReplicaPeer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageSubSystemToReplicaPeer {

/// 
    #[serde(rename = "ReplicaPeer")]
    pub replica_peer: Option<MSFT_ReplicaPeer>,

/// 
    #[serde(rename = "StorageSubSystem")]
    pub storage_sub_system: Option<MSFT_StorageSubSystem>,
}

impl MSFT_StorageSubSystemToReplicaPeer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            replica_peer: None,
            storage_sub_system: None,
        }
    }


    /// Sets the value of ReplicaPeer
    pub fn set_replica_peer(&mut self, value: MSFT_ReplicaPeer) {
        self.replica_peer = Some(value);
    }

    /// Gets the value of ReplicaPeer
    pub fn get_replica_peer(&self) -> Option<&MSFT_ReplicaPeer> {
        self.replica_peer.as_ref()
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

