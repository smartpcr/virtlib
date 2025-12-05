// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_HyperVDynamicMemoryIntegrationService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_HyperVDynamicMemoryIntegrationService {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "MaximumMemoryMbytes")]
    pub maximum_memory_mbytes: Option<u64>,
}

impl Win32_PerfRawData_Counters_HyperVDynamicMemoryIntegrationService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            maximum_memory_mbytes: None,
        }
    }


    /// Sets the value of MaximumMemoryMbytes
    pub fn set_maximum_memory_mbytes(&mut self, value: u64) {
        self.maximum_memory_mbytes = Some(value);
    }

    /// Gets the value of MaximumMemoryMbytes
    pub fn get_maximum_memory_mbytes(&self) -> Option<&u64> {
        self.maximum_memory_mbytes.as_ref()
    }
}

