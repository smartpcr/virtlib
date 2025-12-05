// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualSystemCollection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualSystemCollection {
    #[serde(flatten)]
    pub base: CIM_CollectionOfMSEs,

/// 
    #[serde(rename = "FailedOverReplicationType")]
    pub failed_over_replication_type: Option<u16>,

/// 
    #[serde(rename = "LastApplyConsistencyLevel")]
    pub last_apply_consistency_level: Option<u16>,

/// 
    #[serde(rename = "LastApplyTime")]
    pub last_apply_time: Option<String>,

/// 
    #[serde(rename = "LastApplyVirtualMachineIds")]
    pub last_apply_virtual_machine_ids: Vec<String>,

/// 
    #[serde(rename = "ReplicationMode")]
    pub replication_mode: Option<u16>,

/// 
    #[serde(rename = "ReplicationState")]
    pub replication_state: Option<u16>,
}

impl Msvm_VirtualSystemCollection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_CollectionOfMSEs::new(),
            failed_over_replication_type: None,
            last_apply_consistency_level: None,
            last_apply_time: None,
            last_apply_virtual_machine_ids: Vec::new(),
            replication_mode: None,
            replication_state: None,
        }
    }


    /// Sets the value of FailedOverReplicationType
    pub fn set_failed_over_replication_type(&mut self, value: u16) {
        self.failed_over_replication_type = Some(value);
    }

    /// Gets the value of FailedOverReplicationType
    pub fn get_failed_over_replication_type(&self) -> Option<&u16> {
        self.failed_over_replication_type.as_ref()
    }

    /// Sets the value of LastApplyConsistencyLevel
    pub fn set_last_apply_consistency_level(&mut self, value: u16) {
        self.last_apply_consistency_level = Some(value);
    }

    /// Gets the value of LastApplyConsistencyLevel
    pub fn get_last_apply_consistency_level(&self) -> Option<&u16> {
        self.last_apply_consistency_level.as_ref()
    }

    /// Sets the value of LastApplyTime
    pub fn set_last_apply_time(&mut self, value: String) {
        self.last_apply_time = Some(value);
    }

    /// Gets the value of LastApplyTime
    pub fn get_last_apply_time(&self) -> Option<&String> {
        self.last_apply_time.as_ref()
    }

    /// Sets the value of LastApplyVirtualMachineIds
    pub fn set_last_apply_virtual_machine_ids(&mut self, value: Vec<String>) {
        self.last_apply_virtual_machine_ids = value;
    }

    /// Gets the value of LastApplyVirtualMachineIds
    pub fn get_last_apply_virtual_machine_ids(&self) -> &Vec<String> {
        &self.last_apply_virtual_machine_ids
    }

    /// Sets the value of ReplicationMode
    pub fn set_replication_mode(&mut self, value: u16) {
        self.replication_mode = Some(value);
    }

    /// Gets the value of ReplicationMode
    pub fn get_replication_mode(&self) -> Option<&u16> {
        self.replication_mode.as_ref()
    }

    /// Sets the value of ReplicationState
    pub fn set_replication_state(&mut self, value: u16) {
        self.replication_state = Some(value);
    }

    /// Gets the value of ReplicationState
    pub fn get_replication_state(&self) -> Option<&u16> {
        self.replication_state.as_ref()
    }
}

