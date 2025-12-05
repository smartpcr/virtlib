// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ReplicationGroupToReplicaPeer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ReplicationGroupToReplicaPeer {
    #[serde(flatten)]
    pub base: MSFT_Synchronized,

/// 
    #[serde(rename = "ConsistencyState")]
    pub consistency_state: Option<u16>,

/// 
    #[serde(rename = "ConsistencyType")]
    pub consistency_type: Option<u16>,

/// 
    #[serde(rename = "ReplicaPeer")]
    pub replica_peer: Option<MSFT_ReplicaPeer>,

/// 
    #[serde(rename = "ReplicationGroup")]
    pub replication_group: Option<MSFT_ReplicationGroup>,
}

impl MSFT_ReplicationGroupToReplicaPeer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_Synchronized::new(),
            consistency_state: None,
            consistency_type: None,
            replica_peer: None,
            replication_group: None,
        }
    }


    /// Sets the value of ConsistencyState
    pub fn set_consistency_state(&mut self, value: u16) {
        self.consistency_state = Some(value);
    }

    /// Gets the value of ConsistencyState
    pub fn get_consistency_state(&self) -> Option<&u16> {
        self.consistency_state.as_ref()
    }

    /// Sets the value of ConsistencyType
    pub fn set_consistency_type(&mut self, value: u16) {
        self.consistency_type = Some(value);
    }

    /// Gets the value of ConsistencyType
    pub fn get_consistency_type(&self) -> Option<&u16> {
        self.consistency_type.as_ref()
    }

    /// Sets the value of ReplicaPeer
    pub fn set_replica_peer(&mut self, value: MSFT_ReplicaPeer) {
        self.replica_peer = Some(value);
    }

    /// Gets the value of ReplicaPeer
    pub fn get_replica_peer(&self) -> Option<&MSFT_ReplicaPeer> {
        self.replica_peer.as_ref()
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

