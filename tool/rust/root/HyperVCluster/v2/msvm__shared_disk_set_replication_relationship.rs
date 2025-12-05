// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_SharedDiskSetReplicationRelationship struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_SharedDiskSetReplicationRelationship {

/// 
    #[serde(rename = "Caption")]
    pub caption: Option<String>,

/// 
    #[serde(rename = "CollectionID")]
    pub collection_id: Option<String>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "ElementName")]
    pub element_name: Option<String>,

/// 
    #[serde(rename = "FailedOverReplicationType")]
    pub failed_over_replication_type: Option<u16>,

/// 
    #[serde(rename = "LastApplicationConsistentReplicationTime")]
    pub last_application_consistent_replication_time: Option<String>,

/// 
    #[serde(rename = "LastApplyTime")]
    pub last_apply_time: Option<String>,

/// 
    #[serde(rename = "LastReplicationTime")]
    pub last_replication_time: Option<String>,

/// 
    #[serde(rename = "LastReplicationType")]
    pub last_replication_type: Option<u16>,

/// 
    #[serde(rename = "ReplicationHealth")]
    pub replication_health: Option<u16>,

/// 
    #[serde(rename = "ReplicationState")]
    pub replication_state: Option<u16>,
}

impl Msvm_SharedDiskSetReplicationRelationship {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            caption: None,
            collection_id: None,
            description: None,
            element_name: None,
            failed_over_replication_type: None,
            last_application_consistent_replication_time: None,
            last_apply_time: None,
            last_replication_time: None,
            last_replication_type: None,
            replication_health: None,
            replication_state: None,
        }
    }


    /// Sets the value of Caption
    pub fn set_caption(&mut self, value: String) {
        self.caption = Some(value);
    }

    /// Gets the value of Caption
    pub fn get_caption(&self) -> Option<&String> {
        self.caption.as_ref()
    }

    /// Sets the value of CollectionID
    pub fn set_collection_id(&mut self, value: String) {
        self.collection_id = Some(value);
    }

    /// Gets the value of CollectionID
    pub fn get_collection_id(&self) -> Option<&String> {
        self.collection_id.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of ElementName
    pub fn set_element_name(&mut self, value: String) {
        self.element_name = Some(value);
    }

    /// Gets the value of ElementName
    pub fn get_element_name(&self) -> Option<&String> {
        self.element_name.as_ref()
    }

    /// Sets the value of FailedOverReplicationType
    pub fn set_failed_over_replication_type(&mut self, value: u16) {
        self.failed_over_replication_type = Some(value);
    }

    /// Gets the value of FailedOverReplicationType
    pub fn get_failed_over_replication_type(&self) -> Option<&u16> {
        self.failed_over_replication_type.as_ref()
    }

    /// Sets the value of LastApplicationConsistentReplicationTime
    pub fn set_last_application_consistent_replication_time(&mut self, value: String) {
        self.last_application_consistent_replication_time = Some(value);
    }

    /// Gets the value of LastApplicationConsistentReplicationTime
    pub fn get_last_application_consistent_replication_time(&self) -> Option<&String> {
        self.last_application_consistent_replication_time.as_ref()
    }

    /// Sets the value of LastApplyTime
    pub fn set_last_apply_time(&mut self, value: String) {
        self.last_apply_time = Some(value);
    }

    /// Gets the value of LastApplyTime
    pub fn get_last_apply_time(&self) -> Option<&String> {
        self.last_apply_time.as_ref()
    }

    /// Sets the value of LastReplicationTime
    pub fn set_last_replication_time(&mut self, value: String) {
        self.last_replication_time = Some(value);
    }

    /// Gets the value of LastReplicationTime
    pub fn get_last_replication_time(&self) -> Option<&String> {
        self.last_replication_time.as_ref()
    }

    /// Sets the value of LastReplicationType
    pub fn set_last_replication_type(&mut self, value: u16) {
        self.last_replication_type = Some(value);
    }

    /// Gets the value of LastReplicationType
    pub fn get_last_replication_type(&self) -> Option<&u16> {
        self.last_replication_type.as_ref()
    }

    /// Sets the value of ReplicationHealth
    pub fn set_replication_health(&mut self, value: u16) {
        self.replication_health = Some(value);
    }

    /// Gets the value of ReplicationHealth
    pub fn get_replication_health(&self) -> Option<&u16> {
        self.replication_health.as_ref()
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

