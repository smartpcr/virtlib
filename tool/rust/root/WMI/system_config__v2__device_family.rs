// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V2_DeviceFamily struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V2_DeviceFamily {
    #[serde(flatten)]
    pub base: SystemConfig_V2,

/// 
    #[serde(rename = "DeviceFamily")]
    pub device_family: Option<u32>,

/// 
    #[serde(rename = "DeviceForm")]
    pub device_form: Option<u32>,

/// 
    #[serde(rename = "UAPInfo")]
    pub uapinfo: Option<u64>,
}

impl SystemConfig_V2_DeviceFamily {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V2::new(),
            device_family: None,
            device_form: None,
            uapinfo: None,
        }
    }


    /// Sets the value of DeviceFamily
    pub fn set_device_family(&mut self, value: u32) {
        self.device_family = Some(value);
    }

    /// Gets the value of DeviceFamily
    pub fn get_device_family(&self) -> Option<&u32> {
        self.device_family.as_ref()
    }

    /// Sets the value of DeviceForm
    pub fn set_device_form(&mut self, value: u32) {
        self.device_form = Some(value);
    }

    /// Gets the value of DeviceForm
    pub fn get_device_form(&self) -> Option<&u32> {
        self.device_form.as_ref()
    }

    /// Sets the value of UAPInfo
    pub fn set_uapinfo(&mut self, value: u64) {
        self.uapinfo = Some(value);
    }

    /// Gets the value of UAPInfo
    pub fn get_uapinfo(&self) -> Option<&u64> {
        self.uapinfo.as_ref()
    }
}

