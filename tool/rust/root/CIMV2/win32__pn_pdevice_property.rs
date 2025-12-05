// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PnPDeviceProperty struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PnPDeviceProperty {

/// 
    #[serde(rename = "DeviceID")]
    pub device_id: Option<String>,

/// 
    #[serde(rename = "key")]
    pub key: Option<String>,

/// 
    #[serde(rename = "KeyName")]
    pub key_name: Option<String>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u32>,
}

impl Win32_PnPDeviceProperty {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            device_id: None,
            key: None,
            key_name: None,
            type: None,
        }
    }


    /// Sets the value of DeviceID
    pub fn set_device_id(&mut self, value: String) {
        self.device_id = Some(value);
    }

    /// Gets the value of DeviceID
    pub fn get_device_id(&self) -> Option<&String> {
        self.device_id.as_ref()
    }

    /// Sets the value of key
    pub fn set_key(&mut self, value: String) {
        self.key = Some(value);
    }

    /// Gets the value of key
    pub fn get_key(&self) -> Option<&String> {
        self.key.as_ref()
    }

    /// Sets the value of KeyName
    pub fn set_key_name(&mut self, value: String) {
        self.key_name = Some(value);
    }

    /// Gets the value of KeyName
    pub fn get_key_name(&self) -> Option<&String> {
        self.key_name.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u32) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u32> {
        self.type.as_ref()
    }
}

