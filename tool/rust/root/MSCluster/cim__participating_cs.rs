// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ParticipatingCS struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ParticipatingCS {
    #[serde(flatten)]
    pub base: CIM_Dependency,

/// 
    #[serde(rename = "RoleOfNode")]
    pub role_of_node: Option<u16>,

/// 
    #[serde(rename = "StateOfNode")]
    pub state_of_node: Option<u16>,
}

impl CIM_ParticipatingCS {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Dependency::new(),
            role_of_node: None,
            state_of_node: None,
        }
    }


    /// Sets the value of RoleOfNode
    pub fn set_role_of_node(&mut self, value: u16) {
        self.role_of_node = Some(value);
    }

    /// Gets the value of RoleOfNode
    pub fn get_role_of_node(&self) -> Option<&u16> {
        self.role_of_node.as_ref()
    }

    /// Sets the value of StateOfNode
    pub fn set_state_of_node(&mut self, value: u16) {
        self.state_of_node = Some(value);
    }

    /// Gets the value of StateOfNode
    pub fn get_state_of_node(&self) -> Option<&u16> {
        self.state_of_node.as_ref()
    }
}

