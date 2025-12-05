// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_JobObjectStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_JobObjectStatus {
    #[serde(flatten)]
    pub base: __ExtendedStatus,

/// 
    #[serde(rename = "AdditionalDescription")]
    pub additional_description: Option<String>,

/// 
    #[serde(rename = "Win32ErrorCode")]
    pub win32_error_code: Option<u32>,
}

impl Win32_JobObjectStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ExtendedStatus::new(),
            additional_description: None,
            win32_error_code: None,
        }
    }


    /// Sets the value of AdditionalDescription
    pub fn set_additional_description(&mut self, value: String) {
        self.additional_description = Some(value);
    }

    /// Gets the value of AdditionalDescription
    pub fn get_additional_description(&self) -> Option<&String> {
        self.additional_description.as_ref()
    }

    /// Sets the value of Win32ErrorCode
    pub fn set_win32_error_code(&mut self, value: u32) {
        self.win32_error_code = Some(value);
    }

    /// Gets the value of Win32ErrorCode
    pub fn get_win32_error_code(&self) -> Option<&u32> {
        self.win32_error_code.as_ref()
    }
}

