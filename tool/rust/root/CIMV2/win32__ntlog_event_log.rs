// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_NTLogEventLog struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_NTLogEventLog {

/// 
    #[serde(rename = "Log")]
    pub log: Option<Win32_NTEventlogFile>,

/// 
    #[serde(rename = "Record")]
    pub record: Option<Win32_NTLogEvent>,
}

impl Win32_NTLogEventLog {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            log: None,
            record: None,
        }
    }


    /// Sets the value of Log
    pub fn set_log(&mut self, value: Win32_NTEventlogFile) {
        self.log = Some(value);
    }

    /// Gets the value of Log
    pub fn get_log(&self) -> Option<&Win32_NTEventlogFile> {
        self.log.as_ref()
    }

    /// Sets the value of Record
    pub fn set_record(&mut self, value: Win32_NTLogEvent) {
        self.record = Some(value);
    }

    /// Gets the value of Record
    pub fn get_record(&self) -> Option<&Win32_NTLogEvent> {
        self.record.as_ref()
    }
}

