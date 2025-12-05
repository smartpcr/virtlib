// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_GPUPerformanceCounters_GPULocalAdapterMemory struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_GPUPerformanceCounters_GPULocalAdapterMemory {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "LocalUsage")]
    pub local_usage: Option<u64>,
}

impl Win32_PerfRawData_GPUPerformanceCounters_GPULocalAdapterMemory {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            local_usage: None,
        }
    }


    /// Sets the value of LocalUsage
    pub fn set_local_usage(&mut self, value: u64) {
        self.local_usage = Some(value);
    }

    /// Gets the value of LocalUsage
    pub fn get_local_usage(&self) -> Option<&u64> {
        self.local_usage.as_ref()
    }
}

