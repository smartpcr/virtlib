// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_SyntheticDisplayControllerSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_SyntheticDisplayControllerSettingData {
    #[serde(flatten)]
    pub base: CIM_ResourceAllocationSettingData,

/// 
    #[serde(rename = "HorizontalResolution")]
    pub horizontal_resolution: Option<u16>,

/// 
    #[serde(rename = "ResolutionType")]
    pub resolution_type: Option<u8>,

/// 
    #[serde(rename = "VerticalResolution")]
    pub vertical_resolution: Option<u16>,
}

impl Msvm_SyntheticDisplayControllerSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ResourceAllocationSettingData::new(),
            horizontal_resolution: None,
            resolution_type: None,
            vertical_resolution: None,
        }
    }


    /// Sets the value of HorizontalResolution
    pub fn set_horizontal_resolution(&mut self, value: u16) {
        self.horizontal_resolution = Some(value);
    }

    /// Gets the value of HorizontalResolution
    pub fn get_horizontal_resolution(&self) -> Option<&u16> {
        self.horizontal_resolution.as_ref()
    }

    /// Sets the value of ResolutionType
    pub fn set_resolution_type(&mut self, value: u8) {
        self.resolution_type = Some(value);
    }

    /// Gets the value of ResolutionType
    pub fn get_resolution_type(&self) -> Option<&u8> {
        self.resolution_type.as_ref()
    }

    /// Sets the value of VerticalResolution
    pub fn set_vertical_resolution(&mut self, value: u16) {
        self.vertical_resolution = Some(value);
    }

    /// Gets the value of VerticalResolution
    pub fn get_vertical_resolution(&self) -> Option<&u16> {
        self.vertical_resolution.as_ref()
    }
}

impl Msvm_SyntheticDisplayControllerSettingData {
    /// Gets the related Msvm_AllocationCapabilities object(s)
    pub fn get_related__allocation_capabilities(&self) -> Result<Msvm_AllocationCapabilities, WmiError> {
        self.get_related("Msvm_AllocationCapabilities")
    }

}

