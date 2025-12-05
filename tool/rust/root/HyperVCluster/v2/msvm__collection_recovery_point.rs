// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_CollectionRecoveryPoint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_CollectionRecoveryPoint {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "CollectionID")]
    pub collection_id: Option<String>,

/// 
    #[serde(rename = "ConsistencyLevel")]
    pub consistency_level: Option<u16>,

/// 
    #[serde(rename = "CreationTime")]
    pub creation_time: Option<String>,

/// 
    #[serde(rename = "RecoveryPointIds")]
    pub recovery_point_ids: Vec<String>,

/// 
    #[serde(rename = "VirtualMachineIds")]
    pub virtual_machine_ids: Vec<String>,
}

impl Msvm_CollectionRecoveryPoint {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            collection_id: None,
            consistency_level: None,
            creation_time: None,
            recovery_point_ids: Vec::new(),
            virtual_machine_ids: Vec::new(),
        }
    }


    /// Sets the value of CollectionID
    pub fn set_collection_id(&mut self, value: String) {
        self.collection_id = Some(value);
    }

    /// Gets the value of CollectionID
    pub fn get_collection_id(&self) -> Option<&String> {
        self.collection_id.as_ref()
    }

    /// Sets the value of ConsistencyLevel
    pub fn set_consistency_level(&mut self, value: u16) {
        self.consistency_level = Some(value);
    }

    /// Gets the value of ConsistencyLevel
    pub fn get_consistency_level(&self) -> Option<&u16> {
        self.consistency_level.as_ref()
    }

    /// Sets the value of CreationTime
    pub fn set_creation_time(&mut self, value: String) {
        self.creation_time = Some(value);
    }

    /// Gets the value of CreationTime
    pub fn get_creation_time(&self) -> Option<&String> {
        self.creation_time.as_ref()
    }

    /// Sets the value of RecoveryPointIds
    pub fn set_recovery_point_ids(&mut self, value: Vec<String>) {
        self.recovery_point_ids = value;
    }

    /// Gets the value of RecoveryPointIds
    pub fn get_recovery_point_ids(&self) -> &Vec<String> {
        &self.recovery_point_ids
    }

    /// Sets the value of VirtualMachineIds
    pub fn set_virtual_machine_ids(&mut self, value: Vec<String>) {
        self.virtual_machine_ids = value;
    }

    /// Gets the value of VirtualMachineIds
    pub fn get_virtual_machine_ids(&self) -> &Vec<String> {
        &self.virtual_machine_ids
    }
}

