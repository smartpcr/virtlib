// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_MemoryDevice struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_MemoryDevice {
    #[serde(flatten)]
    pub base: Win32_SMBIOSMemory,

/// 
    #[serde(rename = "ErrorGranularity")]
    pub error_granularity: Option<u16>,
}

impl Win32_MemoryDevice {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_SMBIOSMemory::new(),
            error_granularity: None,
        }
    }


    /// Sets the value of ErrorGranularity
    pub fn set_error_granularity(&mut self, value: u16) {
        self.error_granularity = Some(value);
    }

    /// Gets the value of ErrorGranularity
    pub fn get_error_granularity(&self) -> Option<&u16> {
        self.error_granularity.as_ref()
    }
}

