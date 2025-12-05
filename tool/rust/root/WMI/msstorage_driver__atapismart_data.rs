// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSStorageDriver_ATAPISmartData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSStorageDriver_ATAPISmartData {
    #[serde(flatten)]
    pub base: MSStorageDriver,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "Checksum")]
    pub checksum: Option<u8>,

/// 
    #[serde(rename = "ErrorLogCapability")]
    pub error_log_capability: Option<u8>,

/// 
    #[serde(rename = "ExtendedPollTimeInMinutes")]
    pub extended_poll_time_in_minutes: Option<u8>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "Length")]
    pub length: Option<u32>,

/// 
    #[serde(rename = "OfflineCollectCapability")]
    pub offline_collect_capability: Option<u8>,

/// 
    #[serde(rename = "OfflineCollectionStatus")]
    pub offline_collection_status: Option<ATAPISmartData_OfflineCollectionStatus>,

/// Reserved
    #[serde(rename = "Reserved")]
    pub reserved: Vec<u8>,

/// 
    #[serde(rename = "SelfTestStatus")]
    pub self_test_status: Option<ATAPISmartData_SelfTestStatus>,

/// 
    #[serde(rename = "ShortPollTimeInMinutes")]
    pub short_poll_time_in_minutes: Option<u8>,

/// 
    #[serde(rename = "SmartCapability")]
    pub smart_capability: Option<u16>,

/// 
    #[serde(rename = "TotalTime")]
    pub total_time: Option<u16>,

/// 
    #[serde(rename = "VendorSpecific")]
    pub vendor_specific: Vec<u8>,

/// 
    #[serde(rename = "VendorSpecific2")]
    pub vendor_specific2: Option<u8>,

/// 
    #[serde(rename = "VendorSpecific3")]
    pub vendor_specific3: Option<u8>,

/// 
    #[serde(rename = "VendorSpecific4")]
    pub vendor_specific4: Vec<u8>,
}

impl MSStorageDriver_ATAPISmartData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSStorageDriver::new(),
            active: None,
            checksum: None,
            error_log_capability: None,
            extended_poll_time_in_minutes: None,
            instance_name: None,
            length: None,
            offline_collect_capability: None,
            offline_collection_status: None,
            reserved: Vec::new(),
            self_test_status: None,
            short_poll_time_in_minutes: None,
            smart_capability: None,
            total_time: None,
            vendor_specific: Vec::new(),
            vendor_specific2: None,
            vendor_specific3: None,
            vendor_specific4: Vec::new(),
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of Checksum
    pub fn set_checksum(&mut self, value: u8) {
        self.checksum = Some(value);
    }

    /// Gets the value of Checksum
    pub fn get_checksum(&self) -> Option<&u8> {
        self.checksum.as_ref()
    }

    /// Sets the value of ErrorLogCapability
    pub fn set_error_log_capability(&mut self, value: u8) {
        self.error_log_capability = Some(value);
    }

    /// Gets the value of ErrorLogCapability
    pub fn get_error_log_capability(&self) -> Option<&u8> {
        self.error_log_capability.as_ref()
    }

    /// Sets the value of ExtendedPollTimeInMinutes
    pub fn set_extended_poll_time_in_minutes(&mut self, value: u8) {
        self.extended_poll_time_in_minutes = Some(value);
    }

    /// Gets the value of ExtendedPollTimeInMinutes
    pub fn get_extended_poll_time_in_minutes(&self) -> Option<&u8> {
        self.extended_poll_time_in_minutes.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of Length
    pub fn set_length(&mut self, value: u32) {
        self.length = Some(value);
    }

    /// Gets the value of Length
    pub fn get_length(&self) -> Option<&u32> {
        self.length.as_ref()
    }

    /// Sets the value of OfflineCollectCapability
    pub fn set_offline_collect_capability(&mut self, value: u8) {
        self.offline_collect_capability = Some(value);
    }

    /// Gets the value of OfflineCollectCapability
    pub fn get_offline_collect_capability(&self) -> Option<&u8> {
        self.offline_collect_capability.as_ref()
    }

    /// Sets the value of OfflineCollectionStatus
    pub fn set_offline_collection_status(&mut self, value: ATAPISmartData_OfflineCollectionStatus) {
        self.offline_collection_status = Some(value);
    }

    /// Gets the value of OfflineCollectionStatus
    pub fn get_offline_collection_status(&self) -> Option<&ATAPISmartData_OfflineCollectionStatus> {
        self.offline_collection_status.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: Vec<u8>) {
        self.reserved = value;
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> &Vec<u8> {
        &self.reserved
    }

    /// Sets the value of SelfTestStatus
    pub fn set_self_test_status(&mut self, value: ATAPISmartData_SelfTestStatus) {
        self.self_test_status = Some(value);
    }

    /// Gets the value of SelfTestStatus
    pub fn get_self_test_status(&self) -> Option<&ATAPISmartData_SelfTestStatus> {
        self.self_test_status.as_ref()
    }

    /// Sets the value of ShortPollTimeInMinutes
    pub fn set_short_poll_time_in_minutes(&mut self, value: u8) {
        self.short_poll_time_in_minutes = Some(value);
    }

    /// Gets the value of ShortPollTimeInMinutes
    pub fn get_short_poll_time_in_minutes(&self) -> Option<&u8> {
        self.short_poll_time_in_minutes.as_ref()
    }

    /// Sets the value of SmartCapability
    pub fn set_smart_capability(&mut self, value: u16) {
        self.smart_capability = Some(value);
    }

    /// Gets the value of SmartCapability
    pub fn get_smart_capability(&self) -> Option<&u16> {
        self.smart_capability.as_ref()
    }

    /// Sets the value of TotalTime
    pub fn set_total_time(&mut self, value: u16) {
        self.total_time = Some(value);
    }

    /// Gets the value of TotalTime
    pub fn get_total_time(&self) -> Option<&u16> {
        self.total_time.as_ref()
    }

    /// Sets the value of VendorSpecific
    pub fn set_vendor_specific(&mut self, value: Vec<u8>) {
        self.vendor_specific = value;
    }

    /// Gets the value of VendorSpecific
    pub fn get_vendor_specific(&self) -> &Vec<u8> {
        &self.vendor_specific
    }

    /// Sets the value of VendorSpecific2
    pub fn set_vendor_specific2(&mut self, value: u8) {
        self.vendor_specific2 = Some(value);
    }

    /// Gets the value of VendorSpecific2
    pub fn get_vendor_specific2(&self) -> Option<&u8> {
        self.vendor_specific2.as_ref()
    }

    /// Sets the value of VendorSpecific3
    pub fn set_vendor_specific3(&mut self, value: u8) {
        self.vendor_specific3 = Some(value);
    }

    /// Gets the value of VendorSpecific3
    pub fn get_vendor_specific3(&self) -> Option<&u8> {
        self.vendor_specific3.as_ref()
    }

    /// Sets the value of VendorSpecific4
    pub fn set_vendor_specific4(&mut self, value: Vec<u8>) {
        self.vendor_specific4 = value;
    }

    /// Gets the value of VendorSpecific4
    pub fn get_vendor_specific4(&self) -> &Vec<u8> {
        &self.vendor_specific4
    }
}

