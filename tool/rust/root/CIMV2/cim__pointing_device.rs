// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_PointingDevice struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_PointingDevice {
    #[serde(flatten)]
    pub base: CIM_UserDevice,

/// 
    #[serde(rename = "Handedness")]
    pub handedness: Option<u16>,

/// 
    #[serde(rename = "NumberOfButtons")]
    pub number_of_buttons: Option<u8>,

/// 
    #[serde(rename = "PointingType")]
    pub pointing_type: Option<u16>,

/// 
    #[serde(rename = "Resolution")]
    pub resolution: Option<u32>,
}

impl CIM_PointingDevice {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_UserDevice::new(),
            handedness: None,
            number_of_buttons: None,
            pointing_type: None,
            resolution: None,
        }
    }


    /// Sets the value of Handedness
    pub fn set_handedness(&mut self, value: u16) {
        self.handedness = Some(value);
    }

    /// Gets the value of Handedness
    pub fn get_handedness(&self) -> Option<&u16> {
        self.handedness.as_ref()
    }

    /// Sets the value of NumberOfButtons
    pub fn set_number_of_buttons(&mut self, value: u8) {
        self.number_of_buttons = Some(value);
    }

    /// Gets the value of NumberOfButtons
    pub fn get_number_of_buttons(&self) -> Option<&u8> {
        self.number_of_buttons.as_ref()
    }

    /// Sets the value of PointingType
    pub fn set_pointing_type(&mut self, value: u16) {
        self.pointing_type = Some(value);
    }

    /// Gets the value of PointingType
    pub fn get_pointing_type(&self) -> Option<&u16> {
        self.pointing_type.as_ref()
    }

    /// Sets the value of Resolution
    pub fn set_resolution(&mut self, value: u32) {
        self.resolution = Some(value);
    }

    /// Gets the value of Resolution
    pub fn get_resolution(&self) -> Option<&u32> {
        self.resolution.as_ref()
    }
}

