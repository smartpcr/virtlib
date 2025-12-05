// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_ReceiveFilterFieldParameters struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_ReceiveFilterFieldParameters {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "FieldByteArrayValue")]
    pub field_byte_array_value: Vec<u8>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "FrameHeader")]
    pub frame_header: Option<u32>,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,

/// 
    #[serde(rename = "MacHeaderField")]
    pub mac_header_field: Option<u32>,

/// 
    #[serde(rename = "ReceiveFilterTest")]
    pub receive_filter_test: Option<u32>,

/// 
    #[serde(rename = "ResultByteArrayValue")]
    pub result_byte_array_value: Vec<u8>,
}

impl MSNdis_ReceiveFilterFieldParameters {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            field_byte_array_value: Vec::new(),
            flags: None,
            frame_header: None,
            header: None,
            mac_header_field: None,
            receive_filter_test: None,
            result_byte_array_value: Vec::new(),
        }
    }


    /// Sets the value of FieldByteArrayValue
    pub fn set_field_byte_array_value(&mut self, value: Vec<u8>) {
        self.field_byte_array_value = value;
    }

    /// Gets the value of FieldByteArrayValue
    pub fn get_field_byte_array_value(&self) -> &Vec<u8> {
        &self.field_byte_array_value
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of FrameHeader
    pub fn set_frame_header(&mut self, value: u32) {
        self.frame_header = Some(value);
    }

    /// Gets the value of FrameHeader
    pub fn get_frame_header(&self) -> Option<&u32> {
        self.frame_header.as_ref()
    }

    /// Sets the value of Header
    pub fn set_header(&mut self, value: MSNdis_ObjectHeader) {
        self.header = Some(value);
    }

    /// Gets the value of Header
    pub fn get_header(&self) -> Option<&MSNdis_ObjectHeader> {
        self.header.as_ref()
    }

    /// Sets the value of MacHeaderField
    pub fn set_mac_header_field(&mut self, value: u32) {
        self.mac_header_field = Some(value);
    }

    /// Gets the value of MacHeaderField
    pub fn get_mac_header_field(&self) -> Option<&u32> {
        self.mac_header_field.as_ref()
    }

    /// Sets the value of ReceiveFilterTest
    pub fn set_receive_filter_test(&mut self, value: u32) {
        self.receive_filter_test = Some(value);
    }

    /// Gets the value of ReceiveFilterTest
    pub fn get_receive_filter_test(&self) -> Option<&u32> {
        self.receive_filter_test.as_ref()
    }

    /// Sets the value of ResultByteArrayValue
    pub fn set_result_byte_array_value(&mut self, value: Vec<u8>) {
        self.result_byte_array_value = value;
    }

    /// Gets the value of ResultByteArrayValue
    pub fn get_result_byte_array_value(&self) -> &Vec<u8> {
        &self.result_byte_array_value
    }
}

