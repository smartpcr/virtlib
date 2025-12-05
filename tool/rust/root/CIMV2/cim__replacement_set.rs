// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ReplacementSet struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ReplacementSet {

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl CIM_ReplacementSet {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            description: None,
            name: None,
        }
    }


    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

