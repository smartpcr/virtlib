// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_VirtualDiskToReplicaPeer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_VirtualDiskToReplicaPeer {
    #[serde(flatten)]
    pub base: MSFT_Synchronized,

/// 
    #[serde(rename = "ReplicaPeer")]
    pub replica_peer: Option<MSFT_ReplicaPeer>,

/// 
    #[serde(rename = "VirtualDisk")]
    pub virtual_disk: Option<MSFT_VirtualDisk>,
}

impl MSFT_VirtualDiskToReplicaPeer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_Synchronized::new(),
            replica_peer: None,
            virtual_disk: None,
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

    /// Sets the value of VirtualDisk
    pub fn set_virtual_disk(&mut self, value: MSFT_VirtualDisk) {
        self.virtual_disk = Some(value);
    }

    /// Gets the value of VirtualDisk
    pub fn get_virtual_disk(&self) -> Option<&MSFT_VirtualDisk> {
        self.virtual_disk.as_ref()
    }
}

