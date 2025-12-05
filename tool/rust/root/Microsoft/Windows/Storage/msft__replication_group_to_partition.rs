// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ReplicationGroupToPartition struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ReplicationGroupToPartition {

/// 
    #[serde(rename = "Partition")]
    pub partition: Option<MSFT_Partition>,

/// 
    #[serde(rename = "ReplicationGroup")]
    pub replication_group: Option<MSFT_ReplicationGroup>,
}

impl MSFT_ReplicationGroupToPartition {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            partition: None,
            replication_group: None,
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

    /// Sets the value of ReplicationGroup
    pub fn set_replication_group(&mut self, value: MSFT_ReplicationGroup) {
        self.replication_group = Some(value);
    }

    /// Gets the value of ReplicationGroup
    pub fn get_replication_group(&self) -> Option<&MSFT_ReplicationGroup> {
        self.replication_group.as_ref()
    }
}

