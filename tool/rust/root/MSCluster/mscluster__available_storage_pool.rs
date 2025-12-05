// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_AvailableStoragePool struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_AvailableStoragePool {
    #[serde(flatten)]
    pub base: MSCluster_LogicalElement,

/// 
    #[serde(rename = "Attributes")]
    pub attributes: Option<u64>,

/// 
    #[serde(rename = "ConnectedNodes")]
    pub connected_nodes: Vec<String>,

/// 
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<u32>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "QuorumStatus")]
    pub quorum_status: Option<u32>,

/// 
    #[serde(rename = "TotalSize")]
    pub total_size: Option<u64>,

/// 
    #[serde(rename = "Usage")]
    pub usage: Option<u64>,
}

impl MSCluster_AvailableStoragePool {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSCluster_LogicalElement::new(),
            attributes: None,
            connected_nodes: Vec::new(),
            health_status: None,
            id: None,
            quorum_status: None,
            total_size: None,
            usage: None,
        }
    }


    /// Sets the value of Attributes
    pub fn set_attributes(&mut self, value: u64) {
        self.attributes = Some(value);
    }

    /// Gets the value of Attributes
    pub fn get_attributes(&self) -> Option<&u64> {
        self.attributes.as_ref()
    }

    /// Sets the value of ConnectedNodes
    pub fn set_connected_nodes(&mut self, value: Vec<String>) {
        self.connected_nodes = value;
    }

    /// Gets the value of ConnectedNodes
    pub fn get_connected_nodes(&self) -> &Vec<String> {
        &self.connected_nodes
    }

    /// Sets the value of HealthStatus
    pub fn set_health_status(&mut self, value: u32) {
        self.health_status = Some(value);
    }

    /// Gets the value of HealthStatus
    pub fn get_health_status(&self) -> Option<&u32> {
        self.health_status.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of QuorumStatus
    pub fn set_quorum_status(&mut self, value: u32) {
        self.quorum_status = Some(value);
    }

    /// Gets the value of QuorumStatus
    pub fn get_quorum_status(&self) -> Option<&u32> {
        self.quorum_status.as_ref()
    }

    /// Sets the value of TotalSize
    pub fn set_total_size(&mut self, value: u64) {
        self.total_size = Some(value);
    }

    /// Gets the value of TotalSize
    pub fn get_total_size(&self) -> Option<&u64> {
        self.total_size.as_ref()
    }

    /// Sets the value of Usage
    pub fn set_usage(&mut self, value: u64) {
        self.usage = Some(value);
    }

    /// Gets the value of Usage
    pub fn get_usage(&self) -> Option<&u64> {
        self.usage.as_ref()
    }

/// 
    pub fn add_to_cluster(&self) -> Result<(), WmiError> {
        self.invoke_method("AddToCluster", &[])

    }

}

