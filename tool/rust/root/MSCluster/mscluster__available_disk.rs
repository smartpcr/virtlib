// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_AvailableDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_AvailableDisk {
    #[serde(flatten)]
    pub base: MSCluster_ClusterDisk,

/// 
    #[serde(rename = "ConnectedNodes")]
    pub connected_nodes: Vec<String>,

/// 
    #[serde(rename = "Node")]
    pub node: Option<String>,

/// 
    #[serde(rename = "ResourceName")]
    pub resource_name: Option<String>,
}

impl MSCluster_AvailableDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSCluster_ClusterDisk::new(),
            connected_nodes: Vec::new(),
            node: None,
            resource_name: None,
        }
    }


    /// Sets the value of ConnectedNodes
    pub fn set_connected_nodes(&mut self, value: Vec<String>) {
        self.connected_nodes = value;
    }

    /// Gets the value of ConnectedNodes
    pub fn get_connected_nodes(&self) -> &Vec<String> {
        &self.connected_nodes
    }

    /// Sets the value of Node
    pub fn set_node(&mut self, value: String) {
        self.node = Some(value);
    }

    /// Gets the value of Node
    pub fn get_node(&self) -> Option<&String> {
        self.node.as_ref()
    }

    /// Sets the value of ResourceName
    pub fn set_resource_name(&mut self, value: String) {
        self.resource_name = Some(value);
    }

    /// Gets the value of ResourceName
    pub fn get_resource_name(&self) -> Option<&String> {
        self.resource_name.as_ref()
    }

/// 

    /// * `resource_name` -  (String)

    /// * `path` -  (String)
    pub fn add_to_cluster(&self, resource_name: &String, path: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ResourceName".to_string(), value: resource_name.into() });

        let result = self.invoke_method("AddToCluster", &args)?;
        let path = result.get_value("Path")?;
        Ok(result.return_value)

    }


/// 

    /// * `resource_name` -  (String)

    /// * `path` -  (String)
    pub fn create_storage_resource(&self, resource_name: &String, path: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ResourceName".to_string(), value: resource_name.into() });

        let result = self.invoke_method("CreateStorageResource", &args)?;
        let path = result.get_value("Path")?;
        Ok(result.return_value)

    }

}

