// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_PciExpress struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_PciExpress {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// 
    #[serde(rename = "DeviceInstancePath")]
    pub device_instance_path: Option<String>,

/// 
    #[serde(rename = "FunctionNumber")]
    pub function_number: Option<u16>,

/// 
    #[serde(rename = "LocationPath")]
    pub location_path: Option<String>,
}

impl Msvm_PciExpress {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            device_instance_path: None,
            function_number: None,
            location_path: None,
        }
    }


    /// Sets the value of DeviceInstancePath
    pub fn set_device_instance_path(&mut self, value: String) {
        self.device_instance_path = Some(value);
    }

    /// Gets the value of DeviceInstancePath
    pub fn get_device_instance_path(&self) -> Option<&String> {
        self.device_instance_path.as_ref()
    }

    /// Sets the value of FunctionNumber
    pub fn set_function_number(&mut self, value: u16) {
        self.function_number = Some(value);
    }

    /// Gets the value of FunctionNumber
    pub fn get_function_number(&self) -> Option<&u16> {
        self.function_number.as_ref()
    }

    /// Sets the value of LocationPath
    pub fn set_location_path(&mut self, value: String) {
        self.location_path = Some(value);
    }

    /// Gets the value of LocationPath
    pub fn get_location_path(&self) -> Option<&String> {
        self.location_path.as_ref()
    }
}

