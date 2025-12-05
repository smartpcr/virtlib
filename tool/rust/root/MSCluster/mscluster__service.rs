// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_Service struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_Service {
    #[serde(flatten)]
    pub base: CIM_ClusteringService,

/// 
    #[serde(rename = "NodeHighestVersion")]
    pub node_highest_version: Option<u32>,

/// 
    #[serde(rename = "NodeLowestVersion")]
    pub node_lowest_version: Option<u32>,

/// 
    #[serde(rename = "State")]
    pub state: Option<String>,
}

impl MSCluster_Service {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ClusteringService::new(),
            node_highest_version: None,
            node_lowest_version: None,
            state: None,
        }
    }


    /// Sets the value of NodeHighestVersion
    pub fn set_node_highest_version(&mut self, value: u32) {
        self.node_highest_version = Some(value);
    }

    /// Gets the value of NodeHighestVersion
    pub fn get_node_highest_version(&self) -> Option<&u32> {
        self.node_highest_version.as_ref()
    }

    /// Sets the value of NodeLowestVersion
    pub fn set_node_lowest_version(&mut self, value: u32) {
        self.node_lowest_version = Some(value);
    }

    /// Gets the value of NodeLowestVersion
    pub fn get_node_lowest_version(&self) -> Option<&u32> {
        self.node_lowest_version.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: String) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&String> {
        self.state.as_ref()
    }

/// 
    pub fn start(&self) -> Result<(), WmiError> {
        self.invoke_method("Start", &[])

    }


/// 
    pub fn stop(&self) -> Result<(), WmiError> {
        self.invoke_method("Stop", &[])

    }

}

