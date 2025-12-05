// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SBLTargetDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SBLTargetDisk {

/// 
    #[serde(rename = "CacheMode")]
    pub cache_mode: Option<u32>,

/// 
    #[serde(rename = "CurrentUsage")]
    pub current_usage: Option<u32>,

/// 
    #[serde(rename = "DesiredUsage")]
    pub desired_usage: Option<u32>,

/// 
    #[serde(rename = "DeviceNumber")]
    pub device_number: Option<u32>,

/// 
    #[serde(rename = "Identifier")]
    pub identifier: Option<String>,

/// 
    #[serde(rename = "IsFlash")]
    pub is_flash: Option<bool>,

/// 
    #[serde(rename = "IsSblCacheDevice")]
    pub is_sbl_cache_device: Option<bool>,

/// 
    #[serde(rename = "LastStateChangeTime")]
    pub last_state_change_time: Option<String>,

/// 
    #[serde(rename = "ReadMediaErrorCount")]
    pub read_media_error_count: Option<u64>,

/// 
    #[serde(rename = "ReadTotalErrorCount")]
    pub read_total_error_count: Option<u64>,

/// 
    #[serde(rename = "SblAttributes")]
    pub sbl_attributes: Option<u32>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,

/// 
    #[serde(rename = "WriteMediaErrorCount")]
    pub write_media_error_count: Option<u64>,

/// 
    #[serde(rename = "WriteTotalErrorCount")]
    pub write_total_error_count: Option<u64>,
}

impl MSFT_SBLTargetDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cache_mode: None,
            current_usage: None,
            desired_usage: None,
            device_number: None,
            identifier: None,
            is_flash: None,
            is_sbl_cache_device: None,
            last_state_change_time: None,
            read_media_error_count: None,
            read_total_error_count: None,
            sbl_attributes: None,
            state: None,
            write_media_error_count: None,
            write_total_error_count: None,
        }
    }


    /// Sets the value of CacheMode
    pub fn set_cache_mode(&mut self, value: u32) {
        self.cache_mode = Some(value);
    }

    /// Gets the value of CacheMode
    pub fn get_cache_mode(&self) -> Option<&u32> {
        self.cache_mode.as_ref()
    }

    /// Sets the value of CurrentUsage
    pub fn set_current_usage(&mut self, value: u32) {
        self.current_usage = Some(value);
    }

    /// Gets the value of CurrentUsage
    pub fn get_current_usage(&self) -> Option<&u32> {
        self.current_usage.as_ref()
    }

    /// Sets the value of DesiredUsage
    pub fn set_desired_usage(&mut self, value: u32) {
        self.desired_usage = Some(value);
    }

    /// Gets the value of DesiredUsage
    pub fn get_desired_usage(&self) -> Option<&u32> {
        self.desired_usage.as_ref()
    }

    /// Sets the value of DeviceNumber
    pub fn set_device_number(&mut self, value: u32) {
        self.device_number = Some(value);
    }

    /// Gets the value of DeviceNumber
    pub fn get_device_number(&self) -> Option<&u32> {
        self.device_number.as_ref()
    }

    /// Sets the value of Identifier
    pub fn set_identifier(&mut self, value: String) {
        self.identifier = Some(value);
    }

    /// Gets the value of Identifier
    pub fn get_identifier(&self) -> Option<&String> {
        self.identifier.as_ref()
    }

    /// Sets the value of IsFlash
    pub fn set_is_flash(&mut self, value: bool) {
        self.is_flash = Some(value);
    }

    /// Gets the value of IsFlash
    pub fn get_is_flash(&self) -> Option<&bool> {
        self.is_flash.as_ref()
    }

    /// Sets the value of IsSblCacheDevice
    pub fn set_is_sbl_cache_device(&mut self, value: bool) {
        self.is_sbl_cache_device = Some(value);
    }

    /// Gets the value of IsSblCacheDevice
    pub fn get_is_sbl_cache_device(&self) -> Option<&bool> {
        self.is_sbl_cache_device.as_ref()
    }

    /// Sets the value of LastStateChangeTime
    pub fn set_last_state_change_time(&mut self, value: String) {
        self.last_state_change_time = Some(value);
    }

    /// Gets the value of LastStateChangeTime
    pub fn get_last_state_change_time(&self) -> Option<&String> {
        self.last_state_change_time.as_ref()
    }

    /// Sets the value of ReadMediaErrorCount
    pub fn set_read_media_error_count(&mut self, value: u64) {
        self.read_media_error_count = Some(value);
    }

    /// Gets the value of ReadMediaErrorCount
    pub fn get_read_media_error_count(&self) -> Option<&u64> {
        self.read_media_error_count.as_ref()
    }

    /// Sets the value of ReadTotalErrorCount
    pub fn set_read_total_error_count(&mut self, value: u64) {
        self.read_total_error_count = Some(value);
    }

    /// Gets the value of ReadTotalErrorCount
    pub fn get_read_total_error_count(&self) -> Option<&u64> {
        self.read_total_error_count.as_ref()
    }

    /// Sets the value of SblAttributes
    pub fn set_sbl_attributes(&mut self, value: u32) {
        self.sbl_attributes = Some(value);
    }

    /// Gets the value of SblAttributes
    pub fn get_sbl_attributes(&self) -> Option<&u32> {
        self.sbl_attributes.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u32> {
        self.state.as_ref()
    }

    /// Sets the value of WriteMediaErrorCount
    pub fn set_write_media_error_count(&mut self, value: u64) {
        self.write_media_error_count = Some(value);
    }

    /// Gets the value of WriteMediaErrorCount
    pub fn get_write_media_error_count(&self) -> Option<&u64> {
        self.write_media_error_count.as_ref()
    }

    /// Sets the value of WriteTotalErrorCount
    pub fn set_write_total_error_count(&mut self, value: u64) {
        self.write_total_error_count = Some(value);
    }

    /// Gets the value of WriteTotalErrorCount
    pub fn get_write_total_error_count(&self) -> Option<&u64> {
        self.write_total_error_count.as_ref()
    }
}

