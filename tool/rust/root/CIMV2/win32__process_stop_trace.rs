// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ProcessStopTrace struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ProcessStopTrace {
    #[serde(flatten)]
    pub base: Win32_ProcessTrace,

/// 
    #[serde(rename = "ExitStatus")]
    pub exit_status: Option<u32>,
}

impl Win32_ProcessStopTrace {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_ProcessTrace::new(),
            exit_status: None,
        }
    }


    /// Sets the value of ExitStatus
    pub fn set_exit_status(&mut self, value: u32) {
        self.exit_status = Some(value);
    }

    /// Gets the value of ExitStatus
    pub fn get_exit_status(&self) -> Option<&u32> {
        self.exit_status.as_ref()
    }
}

