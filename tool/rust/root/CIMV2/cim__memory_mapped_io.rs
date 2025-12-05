// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_MemoryMappedIO struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_MemoryMappedIO {
    #[serde(flatten)]
    pub base: CIM_SystemResource,

/// 
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// 
    #[serde(rename = "CSCreationClassName")]
    pub cscreation_class_name: Option<String>,

/// 
    #[serde(rename = "CSName")]
    pub csname: Option<String>,

/// 
    #[serde(rename = "EndingAddress")]
    pub ending_address: Option<u64>,

/// 
    #[serde(rename = "StartingAddress")]
    pub starting_address: Option<u64>,
}

impl CIM_MemoryMappedIO {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SystemResource::new(),
            creation_class_name: None,
            cscreation_class_name: None,
            csname: None,
            ending_address: None,
            starting_address: None,
        }
    }


    /// Sets the value of CreationClassName
    pub fn set_creation_class_name(&mut self, value: String) {
        self.creation_class_name = Some(value);
    }

    /// Gets the value of CreationClassName
    pub fn get_creation_class_name(&self) -> Option<&String> {
        self.creation_class_name.as_ref()
    }

    /// Sets the value of CSCreationClassName
    pub fn set_cscreation_class_name(&mut self, value: String) {
        self.cscreation_class_name = Some(value);
    }

    /// Gets the value of CSCreationClassName
    pub fn get_cscreation_class_name(&self) -> Option<&String> {
        self.cscreation_class_name.as_ref()
    }

    /// Sets the value of CSName
    pub fn set_csname(&mut self, value: String) {
        self.csname = Some(value);
    }

    /// Gets the value of CSName
    pub fn get_csname(&self) -> Option<&String> {
        self.csname.as_ref()
    }

    /// Sets the value of EndingAddress
    pub fn set_ending_address(&mut self, value: u64) {
        self.ending_address = Some(value);
    }

    /// Gets the value of EndingAddress
    pub fn get_ending_address(&self) -> Option<&u64> {
        self.ending_address.as_ref()
    }

    /// Sets the value of StartingAddress
    pub fn set_starting_address(&mut self, value: u64) {
        self.starting_address = Some(value);
    }

    /// Gets the value of StartingAddress
    pub fn get_starting_address(&self) -> Option<&u64> {
        self.starting_address.as_ref()
    }
}

