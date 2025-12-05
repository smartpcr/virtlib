// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PhysicalMemoryArray struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PhysicalMemoryArray {
    #[serde(flatten)]
    pub base: CIM_PhysicalPackage,

/// 
    #[serde(rename = "Location")]
    pub location: Option<u16>,

/// 
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: Option<u32>,

/// 
    #[serde(rename = "MaxCapacityEx")]
    pub max_capacity_ex: Option<u64>,

/// 
    #[serde(rename = "MemoryDevices")]
    pub memory_devices: Option<u16>,

/// 
    #[serde(rename = "MemoryErrorCorrection")]
    pub memory_error_correction: Option<u16>,

/// 
    #[serde(rename = "Use")]
    pub use: Option<u16>,
}

impl Win32_PhysicalMemoryArray {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PhysicalPackage::new(),
            location: None,
            max_capacity: None,
            max_capacity_ex: None,
            memory_devices: None,
            memory_error_correction: None,
            use: None,
        }
    }


    /// Sets the value of Location
    pub fn set_location(&mut self, value: u16) {
        self.location = Some(value);
    }

    /// Gets the value of Location
    pub fn get_location(&self) -> Option<&u16> {
        self.location.as_ref()
    }

    /// Sets the value of MaxCapacity
    pub fn set_max_capacity(&mut self, value: u32) {
        self.max_capacity = Some(value);
    }

    /// Gets the value of MaxCapacity
    pub fn get_max_capacity(&self) -> Option<&u32> {
        self.max_capacity.as_ref()
    }

    /// Sets the value of MaxCapacityEx
    pub fn set_max_capacity_ex(&mut self, value: u64) {
        self.max_capacity_ex = Some(value);
    }

    /// Gets the value of MaxCapacityEx
    pub fn get_max_capacity_ex(&self) -> Option<&u64> {
        self.max_capacity_ex.as_ref()
    }

    /// Sets the value of MemoryDevices
    pub fn set_memory_devices(&mut self, value: u16) {
        self.memory_devices = Some(value);
    }

    /// Gets the value of MemoryDevices
    pub fn get_memory_devices(&self) -> Option<&u16> {
        self.memory_devices.as_ref()
    }

    /// Sets the value of MemoryErrorCorrection
    pub fn set_memory_error_correction(&mut self, value: u16) {
        self.memory_error_correction = Some(value);
    }

    /// Gets the value of MemoryErrorCorrection
    pub fn get_memory_error_correction(&self) -> Option<&u16> {
        self.memory_error_correction.as_ref()
    }

    /// Sets the value of Use
    pub fn set_use(&mut self, value: u16) {
        self.use = Some(value);
    }

    /// Gets the value of Use
    pub fn get_use(&self) -> Option<&u16> {
        self.use.as_ref()
    }
}

