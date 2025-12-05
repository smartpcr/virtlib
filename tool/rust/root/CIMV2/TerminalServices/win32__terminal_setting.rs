// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TerminalSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TerminalSetting {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "TerminalName")]
    pub terminal_name: Option<String>,
}

impl Win32_TerminalSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            terminal_name: None,
        }
    }


    /// Sets the value of TerminalName
    pub fn set_terminal_name(&mut self, value: String) {
        self.terminal_name = Some(value);
    }

    /// Gets the value of TerminalName
    pub fn get_terminal_name(&self) -> Option<&String> {
        self.terminal_name.as_ref()
    }
}

