// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V2_IDEChannel struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V2_IDEChannel {
    #[serde(flatten)]
    pub base: SystemConfig_V2,

/// 
    #[serde(rename = "DeviceTimingMode")]
    pub device_timing_mode: Option<u32>,

/// 
    #[serde(rename = "DeviceType")]
    pub device_type: Option<u32>,

/// 
    #[serde(rename = "LocationInformation")]
    pub location_information: Option<String>,

/// 
    #[serde(rename = "LocationInformationLen")]
    pub location_information_len: Option<u32>,

/// 
    #[serde(rename = "TargetId")]
    pub target_id: Option<u32>,
}

impl SystemConfig_V2_IDEChannel {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V2::new(),
            device_timing_mode: None,
            device_type: None,
            location_information: None,
            location_information_len: None,
            target_id: None,
        }
    }


    /// Sets the value of DeviceTimingMode
    pub fn set_device_timing_mode(&mut self, value: u32) {
        self.device_timing_mode = Some(value);
    }

    /// Gets the value of DeviceTimingMode
    pub fn get_device_timing_mode(&self) -> Option<&u32> {
        self.device_timing_mode.as_ref()
    }

    /// Sets the value of DeviceType
    pub fn set_device_type(&mut self, value: u32) {
        self.device_type = Some(value);
    }

    /// Gets the value of DeviceType
    pub fn get_device_type(&self) -> Option<&u32> {
        self.device_type.as_ref()
    }

    /// Sets the value of LocationInformation
    pub fn set_location_information(&mut self, value: String) {
        self.location_information = Some(value);
    }

    /// Gets the value of LocationInformation
    pub fn get_location_information(&self) -> Option<&String> {
        self.location_information.as_ref()
    }

    /// Sets the value of LocationInformationLen
    pub fn set_location_information_len(&mut self, value: u32) {
        self.location_information_len = Some(value);
    }

    /// Gets the value of LocationInformationLen
    pub fn get_location_information_len(&self) -> Option<&u32> {
        self.location_information_len.as_ref()
    }

    /// Sets the value of TargetId
    pub fn set_target_id(&mut self, value: u32) {
        self.target_id = Some(value);
    }

    /// Gets the value of TargetId
    pub fn get_target_id(&self) -> Option<&u32> {
        self.target_id.as_ref()
    }
}

