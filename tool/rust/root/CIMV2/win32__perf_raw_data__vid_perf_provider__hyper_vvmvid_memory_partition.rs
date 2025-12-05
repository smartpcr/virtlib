// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_VidPerfProvider_HyperVVMVidMemoryPartition struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_VidPerfProvider_HyperVVMVidMemoryPartition {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "Index")]
    pub index: Option<u64>,
}

impl Win32_PerfRawData_VidPerfProvider_HyperVVMVidMemoryPartition {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            index: None,
        }
    }


    /// Sets the value of Index
    pub fn set_index(&mut self, value: u64) {
        self.index = Some(value);
    }

    /// Gets the value of Index
    pub fn get_index(&self) -> Option<&u64> {
        self.index.as_ref()
    }
}

