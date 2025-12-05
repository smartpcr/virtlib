// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSSmBios_SMBiosEventlog struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSSmBios_SMBiosEventlog {
    #[serde(flatten)]
    pub base: MS_SmBios,

/// 
    #[serde(rename = "AccessMethod")]
    pub access_method: Option<u8>,

/// 
    #[serde(rename = "AccessMethodAddress")]
    pub access_method_address: Option<u32>,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "LengthEachLogTypeDesc")]
    pub length_each_log_type_desc: Option<u8>,

/// 
    #[serde(rename = "ListLogTypeDesc")]
    pub list_log_type_desc: Vec<u8>,

/// 
    #[serde(rename = "LogArea")]
    pub log_area: Vec<u8>,

/// 
    #[serde(rename = "LogAreaLength")]
    pub log_area_length: Option<u16>,

/// 
    #[serde(rename = "LogChangeToken")]
    pub log_change_token: Option<u32>,

/// 
    #[serde(rename = "LogDataStart")]
    pub log_data_start: Option<u16>,

/// 
    #[serde(rename = "LogHeaderDescExists")]
    pub log_header_desc_exists: Option<bool>,

/// 
    #[serde(rename = "LogHeaderFormat")]
    pub log_header_format: Option<u8>,

/// 
    #[serde(rename = "LogHeaderStart")]
    pub log_header_start: Option<u16>,

/// 
    #[serde(rename = "LogStatus")]
    pub log_status: Option<u8>,

/// 
    #[serde(rename = "LogTypeDescLength")]
    pub log_type_desc_length: Option<u16>,

/// 
    #[serde(rename = "NumberLogTypeDesc")]
    pub number_log_type_desc: Option<u8>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u8>,
}

impl MSSmBios_SMBiosEventlog {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MS_SmBios::new(),
            access_method: None,
            access_method_address: None,
            active: None,
            instance_name: None,
            length_each_log_type_desc: None,
            list_log_type_desc: Vec::new(),
            log_area: Vec::new(),
            log_area_length: None,
            log_change_token: None,
            log_data_start: None,
            log_header_desc_exists: None,
            log_header_format: None,
            log_header_start: None,
            log_status: None,
            log_type_desc_length: None,
            number_log_type_desc: None,
            reserved: None,
        }
    }


    /// Sets the value of AccessMethod
    pub fn set_access_method(&mut self, value: u8) {
        self.access_method = Some(value);
    }

    /// Gets the value of AccessMethod
    pub fn get_access_method(&self) -> Option<&u8> {
        self.access_method.as_ref()
    }

    /// Sets the value of AccessMethodAddress
    pub fn set_access_method_address(&mut self, value: u32) {
        self.access_method_address = Some(value);
    }

    /// Gets the value of AccessMethodAddress
    pub fn get_access_method_address(&self) -> Option<&u32> {
        self.access_method_address.as_ref()
    }

    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of LengthEachLogTypeDesc
    pub fn set_length_each_log_type_desc(&mut self, value: u8) {
        self.length_each_log_type_desc = Some(value);
    }

    /// Gets the value of LengthEachLogTypeDesc
    pub fn get_length_each_log_type_desc(&self) -> Option<&u8> {
        self.length_each_log_type_desc.as_ref()
    }

    /// Sets the value of ListLogTypeDesc
    pub fn set_list_log_type_desc(&mut self, value: Vec<u8>) {
        self.list_log_type_desc = value;
    }

    /// Gets the value of ListLogTypeDesc
    pub fn get_list_log_type_desc(&self) -> &Vec<u8> {
        &self.list_log_type_desc
    }

    /// Sets the value of LogArea
    pub fn set_log_area(&mut self, value: Vec<u8>) {
        self.log_area = value;
    }

    /// Gets the value of LogArea
    pub fn get_log_area(&self) -> &Vec<u8> {
        &self.log_area
    }

    /// Sets the value of LogAreaLength
    pub fn set_log_area_length(&mut self, value: u16) {
        self.log_area_length = Some(value);
    }

    /// Gets the value of LogAreaLength
    pub fn get_log_area_length(&self) -> Option<&u16> {
        self.log_area_length.as_ref()
    }

    /// Sets the value of LogChangeToken
    pub fn set_log_change_token(&mut self, value: u32) {
        self.log_change_token = Some(value);
    }

    /// Gets the value of LogChangeToken
    pub fn get_log_change_token(&self) -> Option<&u32> {
        self.log_change_token.as_ref()
    }

    /// Sets the value of LogDataStart
    pub fn set_log_data_start(&mut self, value: u16) {
        self.log_data_start = Some(value);
    }

    /// Gets the value of LogDataStart
    pub fn get_log_data_start(&self) -> Option<&u16> {
        self.log_data_start.as_ref()
    }

    /// Sets the value of LogHeaderDescExists
    pub fn set_log_header_desc_exists(&mut self, value: bool) {
        self.log_header_desc_exists = Some(value);
    }

    /// Gets the value of LogHeaderDescExists
    pub fn get_log_header_desc_exists(&self) -> Option<&bool> {
        self.log_header_desc_exists.as_ref()
    }

    /// Sets the value of LogHeaderFormat
    pub fn set_log_header_format(&mut self, value: u8) {
        self.log_header_format = Some(value);
    }

    /// Gets the value of LogHeaderFormat
    pub fn get_log_header_format(&self) -> Option<&u8> {
        self.log_header_format.as_ref()
    }

    /// Sets the value of LogHeaderStart
    pub fn set_log_header_start(&mut self, value: u16) {
        self.log_header_start = Some(value);
    }

    /// Gets the value of LogHeaderStart
    pub fn get_log_header_start(&self) -> Option<&u16> {
        self.log_header_start.as_ref()
    }

    /// Sets the value of LogStatus
    pub fn set_log_status(&mut self, value: u8) {
        self.log_status = Some(value);
    }

    /// Gets the value of LogStatus
    pub fn get_log_status(&self) -> Option<&u8> {
        self.log_status.as_ref()
    }

    /// Sets the value of LogTypeDescLength
    pub fn set_log_type_desc_length(&mut self, value: u16) {
        self.log_type_desc_length = Some(value);
    }

    /// Gets the value of LogTypeDescLength
    pub fn get_log_type_desc_length(&self) -> Option<&u16> {
        self.log_type_desc_length.as_ref()
    }

    /// Sets the value of NumberLogTypeDesc
    pub fn set_number_log_type_desc(&mut self, value: u8) {
        self.number_log_type_desc = Some(value);
    }

    /// Gets the value of NumberLogTypeDesc
    pub fn get_number_log_type_desc(&self) -> Option<&u8> {
        self.number_log_type_desc.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u8) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u8> {
        self.reserved.as_ref()
    }
}

