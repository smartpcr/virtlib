// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ServerFeature struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ServerFeature {

/// 
    #[serde(rename = "ID")]
    pub id: Option<u32>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<u32>,
}

impl Win32_ServerFeature {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            parent_id: None,
        }
    }


    /// Sets the value of ID
    pub fn set_id(&mut self, value: u32) {
        self.id = Some(value);
    }

    /// Gets the value of ID
    pub fn get_id(&self) -> Option<&u32> {
        self.id.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: u32) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&u32> {
        self.parent_id.as_ref()
    }
}

