// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Cluster struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Cluster {
    #[serde(flatten)]
    pub base: CIM_ComputerSystem,

/// 
    #[serde(rename = "MaxNumberOfNodes")]
    pub max_number_of_nodes: Option<u32>,
}

impl CIM_Cluster {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ComputerSystem::new(),
            max_number_of_nodes: None,
        }
    }


    /// Sets the value of MaxNumberOfNodes
    pub fn set_max_number_of_nodes(&mut self, value: u32) {
        self.max_number_of_nodes = Some(value);
    }

    /// Gets the value of MaxNumberOfNodes
    pub fn get_max_number_of_nodes(&self) -> Option<&u32> {
        self.max_number_of_nodes.as_ref()
    }
}

