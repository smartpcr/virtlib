// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TerminalServiceSettingError struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TerminalServiceSettingError {
    #[serde(flatten)]
    pub base: __ExtendedStatus,

/// 
    #[serde(rename = "TerminalServiceMode")]
    pub terminal_service_mode: Option<i32>,
}

impl Win32_TerminalServiceSettingError {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ExtendedStatus::new(),
            terminal_service_mode: None,
        }
    }


    /// Sets the value of TerminalServiceMode
    pub fn set_terminal_service_mode(&mut self, value: i32) {
        self.terminal_service_mode = Some(value);
    }

    /// Gets the value of TerminalServiceMode
    pub fn get_terminal_service_mode(&self) -> Option<&i32> {
        self.terminal_service_mode.as_ref()
    }
}

