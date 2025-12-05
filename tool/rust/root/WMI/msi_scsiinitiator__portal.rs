// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSIInitiator_Portal struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSIInitiator_Portal {

/// 
    #[serde(rename = "Address")]
    pub address: Option<String>,

/// 
    #[serde(rename = "Index")]
    pub index: Option<u32>,

/// 
    #[serde(rename = "Port")]
    pub port: Option<u16>,

/// 
    #[serde(rename = "SymbolicName")]
    pub symbolic_name: Option<String>,
}

impl MSiSCSIInitiator_Portal {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            address: None,
            index: None,
            port: None,
            symbolic_name: None,
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

    /// Sets the value of Index
    pub fn set_index(&mut self, value: u32) {
        self.index = Some(value);
    }

    /// Gets the value of Index
    pub fn get_index(&self) -> Option<&u32> {
        self.index.as_ref()
    }

    /// Sets the value of Port
    pub fn set_port(&mut self, value: u16) {
        self.port = Some(value);
    }

    /// Gets the value of Port
    pub fn get_port(&self) -> Option<&u16> {
        self.port.as_ref()
    }

    /// Sets the value of SymbolicName
    pub fn set_symbolic_name(&mut self, value: String) {
        self.symbolic_name = Some(value);
    }

    /// Gets the value of SymbolicName
    pub fn get_symbolic_name(&self) -> Option<&String> {
        self.symbolic_name.as_ref()
    }
}

