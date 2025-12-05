// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.HardwareManagement
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_PCSVLogRecord struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_PCSVLogRecord {
    #[serde(flatten)]
    pub base: CIM_LogRecord,

/// 
    #[serde(rename = "RawData")]
    pub raw_data: Vec<u8>,
}

impl MSFT_PCSVLogRecord {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogRecord::new(),
            raw_data: Vec::new(),
        }
    }


    /// Sets the value of RawData
    pub fn set_raw_data(&mut self, value: Vec<u8>) {
        self.raw_data = value;
    }

    /// Gets the value of RawData
    pub fn get_raw_data(&self) -> &Vec<u8> {
        &self.raw_data
    }
}

