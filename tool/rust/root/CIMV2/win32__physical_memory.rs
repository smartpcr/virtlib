// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PhysicalMemory struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PhysicalMemory {
    #[serde(flatten)]
    pub base: CIM_PhysicalMemory,

/// 
    #[serde(rename = "Attributes")]
    pub attributes: Option<u32>,

/// 
    #[serde(rename = "ConfiguredClockSpeed")]
    pub configured_clock_speed: Option<u32>,

/// 
    #[serde(rename = "ConfiguredVoltage")]
    pub configured_voltage: Option<u32>,

/// 
    #[serde(rename = "DeviceLocator")]
    pub device_locator: Option<String>,

/// 
    #[serde(rename = "InterleaveDataDepth")]
    pub interleave_data_depth: Option<u16>,

/// 
    #[serde(rename = "MaxVoltage")]
    pub max_voltage: Option<u32>,

/// 
    #[serde(rename = "MinVoltage")]
    pub min_voltage: Option<u32>,

/// 
    #[serde(rename = "SMBIOSMemoryType")]
    pub smbiosmemory_type: Option<u32>,

/// 
    #[serde(rename = "TypeDetail")]
    pub type_detail: Option<u16>,
}

impl Win32_PhysicalMemory {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PhysicalMemory::new(),
            attributes: None,
            configured_clock_speed: None,
            configured_voltage: None,
            device_locator: None,
            interleave_data_depth: None,
            max_voltage: None,
            min_voltage: None,
            smbiosmemory_type: None,
            type_detail: None,
        }
    }


    /// Sets the value of Attributes
    pub fn set_attributes(&mut self, value: u32) {
        self.attributes = Some(value);
    }

    /// Gets the value of Attributes
    pub fn get_attributes(&self) -> Option<&u32> {
        self.attributes.as_ref()
    }

    /// Sets the value of ConfiguredClockSpeed
    pub fn set_configured_clock_speed(&mut self, value: u32) {
        self.configured_clock_speed = Some(value);
    }

    /// Gets the value of ConfiguredClockSpeed
    pub fn get_configured_clock_speed(&self) -> Option<&u32> {
        self.configured_clock_speed.as_ref()
    }

    /// Sets the value of ConfiguredVoltage
    pub fn set_configured_voltage(&mut self, value: u32) {
        self.configured_voltage = Some(value);
    }

    /// Gets the value of ConfiguredVoltage
    pub fn get_configured_voltage(&self) -> Option<&u32> {
        self.configured_voltage.as_ref()
    }

    /// Sets the value of DeviceLocator
    pub fn set_device_locator(&mut self, value: String) {
        self.device_locator = Some(value);
    }

    /// Gets the value of DeviceLocator
    pub fn get_device_locator(&self) -> Option<&String> {
        self.device_locator.as_ref()
    }

    /// Sets the value of InterleaveDataDepth
    pub fn set_interleave_data_depth(&mut self, value: u16) {
        self.interleave_data_depth = Some(value);
    }

    /// Gets the value of InterleaveDataDepth
    pub fn get_interleave_data_depth(&self) -> Option<&u16> {
        self.interleave_data_depth.as_ref()
    }

    /// Sets the value of MaxVoltage
    pub fn set_max_voltage(&mut self, value: u32) {
        self.max_voltage = Some(value);
    }

    /// Gets the value of MaxVoltage
    pub fn get_max_voltage(&self) -> Option<&u32> {
        self.max_voltage.as_ref()
    }

    /// Sets the value of MinVoltage
    pub fn set_min_voltage(&mut self, value: u32) {
        self.min_voltage = Some(value);
    }

    /// Gets the value of MinVoltage
    pub fn get_min_voltage(&self) -> Option<&u32> {
        self.min_voltage.as_ref()
    }

    /// Sets the value of SMBIOSMemoryType
    pub fn set_smbiosmemory_type(&mut self, value: u32) {
        self.smbiosmemory_type = Some(value);
    }

    /// Gets the value of SMBIOSMemoryType
    pub fn get_smbiosmemory_type(&self) -> Option<&u32> {
        self.smbiosmemory_type.as_ref()
    }

    /// Sets the value of TypeDetail
    pub fn set_type_detail(&mut self, value: u16) {
        self.type_detail = Some(value);
    }

    /// Gets the value of TypeDetail
    pub fn get_type_detail(&self) -> Option<&u16> {
        self.type_detail.as_ref()
    }
}

