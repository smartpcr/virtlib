// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ReplicationGroupToVirtualDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ReplicationGroupToVirtualDisk {

/// 
    #[serde(rename = "ReplicationGroup")]
    pub replication_group: Option<MSFT_ReplicationGroup>,

/// 
    #[serde(rename = "VirtualDisk")]
    pub virtual_disk: Option<MSFT_VirtualDisk>,
}

impl MSFT_ReplicationGroupToVirtualDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            replication_group: None,
            virtual_disk: None,
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

    /// Sets the value of VirtualDisk
    pub fn set_virtual_disk(&mut self, value: MSFT_VirtualDisk) {
        self.virtual_disk = Some(value);
    }

    /// Gets the value of VirtualDisk
    pub fn get_virtual_disk(&self) -> Option<&MSFT_VirtualDisk> {
        self.virtual_disk.as_ref()
    }
}

