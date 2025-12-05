// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V0_PnP struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V0_PnP {
    #[serde(flatten)]
    pub base: SystemConfig_V0,

/// 
    #[serde(rename = "DescriptionLength")]
    pub description_length: Option<u32>,

/// 
    #[serde(rename = "DeviceDescription")]
    pub device_description: Option<String>,

/// 
    #[serde(rename = "DeviceID")]
    pub device_id: Option<String>,

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "FriendlyNameLength")]
    pub friendly_name_length: Option<u32>,

/// 
    #[serde(rename = "IDLength")]
    pub idlength: Option<u32>,
}

impl SystemConfig_V0_PnP {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V0::new(),
            description_length: None,
            device_description: None,
            device_id: None,
            friendly_name: None,
            friendly_name_length: None,
            idlength: None,
        }
    }


    /// Sets the value of DescriptionLength
    pub fn set_description_length(&mut self, value: u32) {
        self.description_length = Some(value);
    }

    /// Gets the value of DescriptionLength
    pub fn get_description_length(&self) -> Option<&u32> {
        self.description_length.as_ref()
    }

    /// Sets the value of DeviceDescription
    pub fn set_device_description(&mut self, value: String) {
        self.device_description = Some(value);
    }

    /// Gets the value of DeviceDescription
    pub fn get_device_description(&self) -> Option<&String> {
        self.device_description.as_ref()
    }

    /// Sets the value of DeviceID
    pub fn set_device_id(&mut self, value: String) {
        self.device_id = Some(value);
    }

    /// Gets the value of DeviceID
    pub fn get_device_id(&self) -> Option<&String> {
        self.device_id.as_ref()
    }

    /// Sets the value of FriendlyName
    pub fn set_friendly_name(&mut self, value: String) {
        self.friendly_name = Some(value);
    }

    /// Gets the value of FriendlyName
    pub fn get_friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    /// Sets the value of FriendlyNameLength
    pub fn set_friendly_name_length(&mut self, value: u32) {
        self.friendly_name_length = Some(value);
    }

    /// Gets the value of FriendlyNameLength
    pub fn get_friendly_name_length(&self) -> Option<&u32> {
        self.friendly_name_length.as_ref()
    }

    /// Sets the value of IDLength
    pub fn set_idlength(&mut self, value: u32) {
        self.idlength = Some(value);
    }

    /// Gets the value of IDLength
    pub fn get_idlength(&self) -> Option<&u32> {
        self.idlength.as_ref()
    }
}

