// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.power
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PowerSettingDefinitionPossibleValue struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PowerSettingDefinitionPossibleValue {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "BinaryValue")]
    pub binary_value: Vec<u8>,

/// 
    #[serde(rename = "SettingIndex")]
    pub setting_index: Option<u32>,

/// 
    #[serde(rename = "StringValue")]
    pub string_value: Option<String>,

/// 
    #[serde(rename = "UInt32Value")]
    pub uint32_value: Option<u32>,

/// 
    #[serde(rename = "UInt64Value")]
    pub uint64_value: Option<u64>,
}

impl Win32_PowerSettingDefinitionPossibleValue {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            binary_value: Vec::new(),
            setting_index: None,
            string_value: None,
            uint32_value: None,
            uint64_value: None,
        }
    }


    /// Sets the value of BinaryValue
    pub fn set_binary_value(&mut self, value: Vec<u8>) {
        self.binary_value = value;
    }

    /// Gets the value of BinaryValue
    pub fn get_binary_value(&self) -> &Vec<u8> {
        &self.binary_value
    }

    /// Sets the value of SettingIndex
    pub fn set_setting_index(&mut self, value: u32) {
        self.setting_index = Some(value);
    }

    /// Gets the value of SettingIndex
    pub fn get_setting_index(&self) -> Option<&u32> {
        self.setting_index.as_ref()
    }

    /// Sets the value of StringValue
    pub fn set_string_value(&mut self, value: String) {
        self.string_value = Some(value);
    }

    /// Gets the value of StringValue
    pub fn get_string_value(&self) -> Option<&String> {
        self.string_value.as_ref()
    }

    /// Sets the value of UInt32Value
    pub fn set_uint32_value(&mut self, value: u32) {
        self.uint32_value = Some(value);
    }

    /// Gets the value of UInt32Value
    pub fn get_uint32_value(&self) -> Option<&u32> {
        self.uint32_value.as_ref()
    }

    /// Sets the value of UInt64Value
    pub fn set_uint64_value(&mut self, value: u64) {
        self.uint64_value = Some(value);
    }

    /// Gets the value of UInt64Value
    pub fn get_uint64_value(&self) -> Option<&u64> {
        self.uint64_value.as_ref()
    }
}

