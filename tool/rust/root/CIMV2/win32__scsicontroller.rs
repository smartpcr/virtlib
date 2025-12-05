// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_SCSIController struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_SCSIController {
    #[serde(flatten)]
    pub base: CIM_SCSIController,

/// 
    #[serde(rename = "DeviceMap")]
    pub device_map: Option<String>,

/// 
    #[serde(rename = "DriverName")]
    pub driver_name: Option<String>,

/// 
    #[serde(rename = "HardwareVersion")]
    pub hardware_version: Option<String>,

/// 
    #[serde(rename = "Index")]
    pub index: Option<u32>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,
}

impl Win32_SCSIController {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SCSIController::new(),
            device_map: None,
            driver_name: None,
            hardware_version: None,
            index: None,
            manufacturer: None,
        }
    }


    /// Sets the value of DeviceMap
    pub fn set_device_map(&mut self, value: String) {
        self.device_map = Some(value);
    }

    /// Gets the value of DeviceMap
    pub fn get_device_map(&self) -> Option<&String> {
        self.device_map.as_ref()
    }

    /// Sets the value of DriverName
    pub fn set_driver_name(&mut self, value: String) {
        self.driver_name = Some(value);
    }

    /// Gets the value of DriverName
    pub fn get_driver_name(&self) -> Option<&String> {
        self.driver_name.as_ref()
    }

    /// Sets the value of HardwareVersion
    pub fn set_hardware_version(&mut self, value: String) {
        self.hardware_version = Some(value);
    }

    /// Gets the value of HardwareVersion
    pub fn get_hardware_version(&self) -> Option<&String> {
        self.hardware_version.as_ref()
    }

    /// Sets the value of Index
    pub fn set_index(&mut self, value: u32) {
        self.index = Some(value);
    }

    /// Gets the value of Index
    pub fn get_index(&self) -> Option<&u32> {
        self.index.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }
}

