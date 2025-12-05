// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_ExtensionStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_ExtensionStatus {

/// 
    #[serde(rename = "beginTime")]
    pub begin_time: Option<String>,

/// 
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,

/// 
    #[serde(rename = "error")]
    pub error: Option<u32>,

/// 
    #[serde(rename = "extensionGuid")]
    pub extension_guid: Option<String>,

/// 
    #[serde(rename = "loggingStatus")]
    pub logging_status: Option<u32>,
}

impl RSOP_ExtensionStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            begin_time: None,
            display_name: None,
            end_time: None,
            error: None,
            extension_guid: None,
            logging_status: None,
        }
    }


    /// Sets the value of beginTime
    pub fn set_begin_time(&mut self, value: String) {
        self.begin_time = Some(value);
    }

    /// Gets the value of beginTime
    pub fn get_begin_time(&self) -> Option<&String> {
        self.begin_time.as_ref()
    }

    /// Sets the value of displayName
    pub fn set_display_name(&mut self, value: String) {
        self.display_name = Some(value);
    }

    /// Gets the value of displayName
    pub fn get_display_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }

    /// Sets the value of endTime
    pub fn set_end_time(&mut self, value: String) {
        self.end_time = Some(value);
    }

    /// Gets the value of endTime
    pub fn get_end_time(&self) -> Option<&String> {
        self.end_time.as_ref()
    }

    /// Sets the value of error
    pub fn set_error(&mut self, value: u32) {
        self.error = Some(value);
    }

    /// Gets the value of error
    pub fn get_error(&self) -> Option<&u32> {
        self.error.as_ref()
    }

    /// Sets the value of extensionGuid
    pub fn set_extension_guid(&mut self, value: String) {
        self.extension_guid = Some(value);
    }

    /// Gets the value of extensionGuid
    pub fn get_extension_guid(&self) -> Option<&String> {
        self.extension_guid.as_ref()
    }

    /// Sets the value of loggingStatus
    pub fn set_logging_status(&mut self, value: u32) {
        self.logging_status = Some(value);
    }

    /// Gets the value of loggingStatus
    pub fn get_logging_status(&self) -> Option<&u32> {
        self.logging_status.as_ref()
    }
}

