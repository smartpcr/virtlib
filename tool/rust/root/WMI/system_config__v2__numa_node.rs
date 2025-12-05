// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V2_NumaNode struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V2_NumaNode {
    #[serde(flatten)]
    pub base: SystemConfig_V2,

/// 
    #[serde(rename = "NodeCount")]
    pub node_count: Option<u32>,

/// 
    #[serde(rename = "NodeMap")]
    pub node_map: Vec<u64>,
}

impl SystemConfig_V2_NumaNode {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V2::new(),
            node_count: None,
            node_map: Vec::new(),
        }
    }


    /// Sets the value of NodeCount
    pub fn set_node_count(&mut self, value: u32) {
        self.node_count = Some(value);
    }

    /// Gets the value of NodeCount
    pub fn get_node_count(&self) -> Option<&u32> {
        self.node_count.as_ref()
    }

    /// Sets the value of NodeMap
    pub fn set_node_map(&mut self, value: Vec<u64>) {
        self.node_map = value;
    }

    /// Gets the value of NodeMap
    pub fn get_node_map(&self) -> &Vec<u64> {
        &self.node_map
    }
}

