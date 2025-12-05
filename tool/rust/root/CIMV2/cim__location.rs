// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Location struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Location {

/// 
    #[serde(rename = "Address")]
    pub address: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "PhysicalPosition")]
    pub physical_position: Option<String>,
}

impl CIM_Location {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            address: None,
            name: None,
            physical_position: None,
        }
    }


    /// Sets the value of Address
    pub fn set_address(&mut self, value: String) {
        self.address = Some(value);
    }

    /// Gets the value of Address
    pub fn get_address(&self) -> Option<&String> {
        self.address.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of PhysicalPosition
    pub fn set_physical_position(&mut self, value: String) {
        self.physical_position = Some(value);
    }

    /// Gets the value of PhysicalPosition
    pub fn get_physical_position(&self) -> Option<&String> {
        self.physical_position.as_ref()
    }
}

