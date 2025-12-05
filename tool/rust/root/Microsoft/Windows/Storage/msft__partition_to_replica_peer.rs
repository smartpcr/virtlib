// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_PartitionToReplicaPeer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_PartitionToReplicaPeer {
    #[serde(flatten)]
    pub base: MSFT_Synchronized,

/// 
    #[serde(rename = "Partition")]
    pub partition: Option<MSFT_Partition>,

/// 
    #[serde(rename = "ReplicaPeer")]
    pub replica_peer: Option<MSFT_ReplicaPeer>,
}

impl MSFT_PartitionToReplicaPeer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_Synchronized::new(),
            partition: None,
            replica_peer: None,
        }
    }


    /// Sets the value of Partition
    pub fn set_partition(&mut self, value: MSFT_Partition) {
        self.partition = Some(value);
    }

    /// Gets the value of Partition
    pub fn get_partition(&self) -> Option<&MSFT_Partition> {
        self.partition.as_ref()
    }

    /// Sets the value of ReplicaPeer
    pub fn set_replica_peer(&mut self, value: MSFT_ReplicaPeer) {
        self.replica_peer = Some(value);
    }

    /// Gets the value of ReplicaPeer
    pub fn get_replica_peer(&self) -> Option<&MSFT_ReplicaPeer> {
        self.replica_peer.as_ref()
    }
}

