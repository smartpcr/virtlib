// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_HeartbeatComponentSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_HeartbeatComponentSettingData {
    #[serde(flatten)]
    pub base: CIM_ResourceAllocationSettingData,

/// 
    #[serde(rename = "EnabledState")]
    pub enabled_state: Option<u16>,

/// 
    #[serde(rename = "ErrorThreshold")]
    pub error_threshold: Option<u32>,

/// 
    #[serde(rename = "Interval")]
    pub interval: Option<u32>,

/// 
    #[serde(rename = "Latency")]
    pub latency: Option<u32>,
}

impl Msvm_HeartbeatComponentSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ResourceAllocationSettingData::new(),
            enabled_state: None,
            error_threshold: None,
            interval: None,
            latency: None,
        }
    }


    /// Sets the value of EnabledState
    pub fn set_enabled_state(&mut self, value: u16) {
        self.enabled_state = Some(value);
    }

    /// Gets the value of EnabledState
    pub fn get_enabled_state(&self) -> Option<&u16> {
        self.enabled_state.as_ref()
    }

    /// Sets the value of ErrorThreshold
    pub fn set_error_threshold(&mut self, value: u32) {
        self.error_threshold = Some(value);
    }

    /// Gets the value of ErrorThreshold
    pub fn get_error_threshold(&self) -> Option<&u32> {
        self.error_threshold.as_ref()
    }

    /// Sets the value of Interval
    pub fn set_interval(&mut self, value: u32) {
        self.interval = Some(value);
    }

    /// Gets the value of Interval
    pub fn get_interval(&self) -> Option<&u32> {
        self.interval.as_ref()
    }

    /// Sets the value of Latency
    pub fn set_latency(&mut self, value: u32) {
        self.latency = Some(value);
    }

    /// Gets the value of Latency
    pub fn get_latency(&self) -> Option<&u32> {
        self.latency.as_ref()
    }
}

impl Msvm_HeartbeatComponentSettingData {
    /// Gets the related Msvm_VirtualSystemSettingData object(s)
    pub fn get_related__virtual_system_setting_data(&self) -> Result<Msvm_VirtualSystemSettingData, WmiError> {
        self.get_related("Msvm_VirtualSystemSettingData")
    }

}

