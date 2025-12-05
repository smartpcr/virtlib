// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_CommandLineAccess struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_CommandLineAccess {
    #[serde(flatten)]
    pub base: CIM_ServiceAccessPoint,

/// 
    #[serde(rename = "CommandLine")]
    pub command_line: Option<String>,
}

impl Win32_CommandLineAccess {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ServiceAccessPoint::new(),
            command_line: None,
        }
    }


    /// Sets the value of CommandLine
    pub fn set_command_line(&mut self, value: String) {
        self.command_line = Some(value);
    }

    /// Gets the value of CommandLine
    pub fn get_command_line(&self) -> Option<&String> {
        self.command_line.as_ref()
    }
}

