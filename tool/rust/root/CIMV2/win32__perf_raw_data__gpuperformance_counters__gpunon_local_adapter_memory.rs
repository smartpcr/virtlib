// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_GPUPerformanceCounters_GPUNonLocalAdapterMemory struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_GPUPerformanceCounters_GPUNonLocalAdapterMemory {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "NonLocalUsage")]
    pub non_local_usage: Option<u64>,
}

impl Win32_PerfRawData_GPUPerformanceCounters_GPUNonLocalAdapterMemory {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            non_local_usage: None,
        }
    }


    /// Sets the value of NonLocalUsage
    pub fn set_non_local_usage(&mut self, value: u64) {
        self.non_local_usage = Some(value);
    }

    /// Gets the value of NonLocalUsage
    pub fn get_non_local_usage(&self) -> Option<&u64> {
        self.non_local_usage.as_ref()
    }
}

