// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_KeyValueStore struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_KeyValueStore {

/// 
    #[serde(rename = "Manager")]
    pub manager: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u32>,
}

impl MSCluster_KeyValueStore {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            manager: None,
            name: None,
            type: None,
        }
    }


    /// Sets the value of Manager
    pub fn set_manager(&mut self, value: String) {
        self.manager = Some(value);
    }

    /// Gets the value of Manager
    pub fn get_manager(&self) -> Option<&String> {
        self.manager.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u32) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u32> {
        self.type.as_ref()
    }
}

