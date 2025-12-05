// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSMCAInfo_Entry struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSMCAInfo_Entry {
    #[serde(flatten)]
    pub base: MSMCAInfo,

/// 
    #[serde(rename = "Data")]
    pub data: Vec<u8>,

/// 
    #[serde(rename = "Length")]
    pub length: Option<u32>,
}

impl MSMCAInfo_Entry {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSMCAInfo::new(),
            data: Vec::new(),
            length: None,
        }
    }


    /// Sets the value of Data
    pub fn set_data(&mut self, value: Vec<u8>) {
        self.data = value;
    }

    /// Gets the value of Data
    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    /// Sets the value of Length
    pub fn set_length(&mut self, value: u32) {
        self.length = Some(value);
    }

    /// Gets the value of Length
    pub fn get_length(&self) -> Option<&u32> {
        self.length.as_ref()
    }
}

