// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_StorageError struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_StorageError {

/// 
    #[serde(rename = "DeviceCreationClassName")]
    pub device_creation_class_name: Option<String>,

/// 
    #[serde(rename = "DeviceID")]
    pub device_id: Option<String>,

/// 
    #[serde(rename = "EndingAddress")]
    pub ending_address: Option<u64>,

/// 
    #[serde(rename = "StartingAddress")]
    pub starting_address: Option<u64>,

/// 
    #[serde(rename = "SystemCreationClassName")]
    pub system_creation_class_name: Option<String>,

/// 
    #[serde(rename = "SystemName")]
    pub system_name: Option<String>,
}

impl CIM_StorageError {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            device_creation_class_name: None,
            device_id: None,
            ending_address: None,
            starting_address: None,
            system_creation_class_name: None,
            system_name: None,
        }
    }


    /// Sets the value of DeviceCreationClassName
    pub fn set_device_creation_class_name(&mut self, value: String) {
        self.device_creation_class_name = Some(value);
    }

    /// Gets the value of DeviceCreationClassName
    pub fn get_device_creation_class_name(&self) -> Option<&String> {
        self.device_creation_class_name.as_ref()
    }

    /// Sets the value of DeviceID
    pub fn set_device_id(&mut self, value: String) {
        self.device_id = Some(value);
    }

    /// Gets the value of DeviceID
    pub fn get_device_id(&self) -> Option<&String> {
        self.device_id.as_ref()
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

    /// Sets the value of SystemCreationClassName
    pub fn set_system_creation_class_name(&mut self, value: String) {
        self.system_creation_class_name = Some(value);
    }

    /// Gets the value of SystemCreationClassName
    pub fn get_system_creation_class_name(&self) -> Option<&String> {
        self.system_creation_class_name.as_ref()
    }

    /// Sets the value of SystemName
    pub fn set_system_name(&mut self, value: String) {
        self.system_name = Some(value);
    }

    /// Gets the value of SystemName
    pub fn get_system_name(&self) -> Option<&String> {
        self.system_name.as_ref()
    }
}

