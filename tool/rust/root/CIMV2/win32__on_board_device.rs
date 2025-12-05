// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_OnBoardDevice struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_OnBoardDevice {
    #[serde(flatten)]
    pub base: CIM_PhysicalComponent,

/// 
    #[serde(rename = "DeviceType")]
    pub device_type: Option<u16>,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
}

impl Win32_OnBoardDevice {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PhysicalComponent::new(),
            device_type: None,
            enabled: None,
        }
    }


    /// Sets the value of DeviceType
    pub fn set_device_type(&mut self, value: u16) {
        self.device_type = Some(value);
    }

    /// Gets the value of DeviceType
    pub fn get_device_type(&self) -> Option<&u16> {
        self.device_type.as_ref()
    }

    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }
}

