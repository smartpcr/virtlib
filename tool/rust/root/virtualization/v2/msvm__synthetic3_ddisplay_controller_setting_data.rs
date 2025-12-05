// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_Synthetic3DDisplayControllerSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_Synthetic3DDisplayControllerSettingData {
    #[serde(flatten)]
    pub base: CIM_ResourceAllocationSettingData,

/// 
    #[serde(rename = "MaximumMonitors")]
    pub maximum_monitors: Option<u8>,

/// 
    #[serde(rename = "MaximumScreenResolution")]
    pub maximum_screen_resolution: Option<u8>,

/// The video memory size for the Virtual Machine
    #[serde(rename = "VRAMSizeBytes")]
    pub vramsize_bytes: Option<u64>,
}

impl Msvm_Synthetic3DDisplayControllerSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ResourceAllocationSettingData::new(),
            maximum_monitors: None,
            maximum_screen_resolution: None,
            vramsize_bytes: None,
        }
    }


    /// Sets the value of MaximumMonitors
    pub fn set_maximum_monitors(&mut self, value: u8) {
        self.maximum_monitors = Some(value);
    }

    /// Gets the value of MaximumMonitors
    pub fn get_maximum_monitors(&self) -> Option<&u8> {
        self.maximum_monitors.as_ref()
    }

    /// Sets the value of MaximumScreenResolution
    pub fn set_maximum_screen_resolution(&mut self, value: u8) {
        self.maximum_screen_resolution = Some(value);
    }

    /// Gets the value of MaximumScreenResolution
    pub fn get_maximum_screen_resolution(&self) -> Option<&u8> {
        self.maximum_screen_resolution.as_ref()
    }

    /// Sets the value of VRAMSizeBytes
    pub fn set_vramsize_bytes(&mut self, value: u64) {
        self.vramsize_bytes = Some(value);
    }

    /// Gets the value of VRAMSizeBytes
    pub fn get_vramsize_bytes(&self) -> Option<&u64> {
        self.vramsize_bytes.as_ref()
    }
}

impl Msvm_Synthetic3DDisplayControllerSettingData {
    /// Gets the related Msvm_AllocationCapabilities object(s)
    pub fn get_related__allocation_capabilities(&self) -> Result<Msvm_AllocationCapabilities, WmiError> {
        self.get_related("Msvm_AllocationCapabilities")
    }

}

