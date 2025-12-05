// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_SMBIOSMemory struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_SMBIOSMemory {
    #[serde(flatten)]
    pub base: CIM_StorageExtent,

/// 
    #[serde(rename = "AdditionalErrorData")]
    pub additional_error_data: Vec<u8>,

/// 
    #[serde(rename = "CorrectableError")]
    pub correctable_error: Option<bool>,

/// 
    #[serde(rename = "EndingAddress")]
    pub ending_address: Option<u64>,

/// 
    #[serde(rename = "ErrorAccess")]
    pub error_access: Option<u16>,

/// 
    #[serde(rename = "ErrorAddress")]
    pub error_address: Option<u64>,

/// 
    #[serde(rename = "ErrorData")]
    pub error_data: Vec<u8>,

/// 
    #[serde(rename = "ErrorDataOrder")]
    pub error_data_order: Option<u16>,

/// 
    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<u16>,

/// 
    #[serde(rename = "ErrorResolution")]
    pub error_resolution: Option<u64>,

/// 
    #[serde(rename = "ErrorTime")]
    pub error_time: Option<String>,

/// 
    #[serde(rename = "ErrorTransferSize")]
    pub error_transfer_size: Option<u32>,

/// 
    #[serde(rename = "OtherErrorDescription")]
    pub other_error_description: Option<String>,

/// 
    #[serde(rename = "StartingAddress")]
    pub starting_address: Option<u64>,

/// 
    #[serde(rename = "SystemLevelAddress")]
    pub system_level_address: Option<bool>,
}

impl Win32_SMBIOSMemory {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_StorageExtent::new(),
            additional_error_data: Vec::new(),
            correctable_error: None,
            ending_address: None,
            error_access: None,
            error_address: None,
            error_data: Vec::new(),
            error_data_order: None,
            error_info: None,
            error_resolution: None,
            error_time: None,
            error_transfer_size: None,
            other_error_description: None,
            starting_address: None,
            system_level_address: None,
        }
    }


    /// Sets the value of AdditionalErrorData
    pub fn set_additional_error_data(&mut self, value: Vec<u8>) {
        self.additional_error_data = value;
    }

    /// Gets the value of AdditionalErrorData
    pub fn get_additional_error_data(&self) -> &Vec<u8> {
        &self.additional_error_data
    }

    /// Sets the value of CorrectableError
    pub fn set_correctable_error(&mut self, value: bool) {
        self.correctable_error = Some(value);
    }

    /// Gets the value of CorrectableError
    pub fn get_correctable_error(&self) -> Option<&bool> {
        self.correctable_error.as_ref()
    }

    /// Sets the value of EndingAddress
    pub fn set_ending_address(&mut self, value: u64) {
        self.ending_address = Some(value);
    }

    /// Gets the value of EndingAddress
    pub fn get_ending_address(&self) -> Option<&u64> {
        self.ending_address.as_ref()
    }

    /// Sets the value of ErrorAccess
    pub fn set_error_access(&mut self, value: u16) {
        self.error_access = Some(value);
    }

    /// Gets the value of ErrorAccess
    pub fn get_error_access(&self) -> Option<&u16> {
        self.error_access.as_ref()
    }

    /// Sets the value of ErrorAddress
    pub fn set_error_address(&mut self, value: u64) {
        self.error_address = Some(value);
    }

    /// Gets the value of ErrorAddress
    pub fn get_error_address(&self) -> Option<&u64> {
        self.error_address.as_ref()
    }

    /// Sets the value of ErrorData
    pub fn set_error_data(&mut self, value: Vec<u8>) {
        self.error_data = value;
    }

    /// Gets the value of ErrorData
    pub fn get_error_data(&self) -> &Vec<u8> {
        &self.error_data
    }

    /// Sets the value of ErrorDataOrder
    pub fn set_error_data_order(&mut self, value: u16) {
        self.error_data_order = Some(value);
    }

    /// Gets the value of ErrorDataOrder
    pub fn get_error_data_order(&self) -> Option<&u16> {
        self.error_data_order.as_ref()
    }

    /// Sets the value of ErrorInfo
    pub fn set_error_info(&mut self, value: u16) {
        self.error_info = Some(value);
    }

    /// Gets the value of ErrorInfo
    pub fn get_error_info(&self) -> Option<&u16> {
        self.error_info.as_ref()
    }

    /// Sets the value of ErrorResolution
    pub fn set_error_resolution(&mut self, value: u64) {
        self.error_resolution = Some(value);
    }

    /// Gets the value of ErrorResolution
    pub fn get_error_resolution(&self) -> Option<&u64> {
        self.error_resolution.as_ref()
    }

    /// Sets the value of ErrorTime
    pub fn set_error_time(&mut self, value: String) {
        self.error_time = Some(value);
    }

    /// Gets the value of ErrorTime
    pub fn get_error_time(&self) -> Option<&String> {
        self.error_time.as_ref()
    }

    /// Sets the value of ErrorTransferSize
    pub fn set_error_transfer_size(&mut self, value: u32) {
        self.error_transfer_size = Some(value);
    }

    /// Gets the value of ErrorTransferSize
    pub fn get_error_transfer_size(&self) -> Option<&u32> {
        self.error_transfer_size.as_ref()
    }

    /// Sets the value of OtherErrorDescription
    pub fn set_other_error_description(&mut self, value: String) {
        self.other_error_description = Some(value);
    }

    /// Gets the value of OtherErrorDescription
    pub fn get_other_error_description(&self) -> Option<&String> {
        self.other_error_description.as_ref()
    }

    /// Sets the value of StartingAddress
    pub fn set_starting_address(&mut self, value: u64) {
        self.starting_address = Some(value);
    }

    /// Gets the value of StartingAddress
    pub fn get_starting_address(&self) -> Option<&u64> {
        self.starting_address.as_ref()
    }

    /// Sets the value of SystemLevelAddress
    pub fn set_system_level_address(&mut self, value: bool) {
        self.system_level_address = Some(value);
    }

    /// Gets the value of SystemLevelAddress
    pub fn get_system_level_address(&self) -> Option<&bool> {
        self.system_level_address.as_ref()
    }
}

