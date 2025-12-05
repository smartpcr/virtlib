// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_GPUPerformanceCounters_GPUProcessMemory struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_GPUPerformanceCounters_GPUProcessMemory {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "DedicatedUsage")]
    pub dedicated_usage: Option<u64>,

/// 
    #[serde(rename = "LocalUsage")]
    pub local_usage: Option<u64>,

/// 
    #[serde(rename = "NonLocalUsage")]
    pub non_local_usage: Option<u64>,

/// 
    #[serde(rename = "SharedUsage")]
    pub shared_usage: Option<u64>,

/// 
    #[serde(rename = "TotalCommitted")]
    pub total_committed: Option<u64>,
}

impl Win32_PerfFormattedData_GPUPerformanceCounters_GPUProcessMemory {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            dedicated_usage: None,
            local_usage: None,
            non_local_usage: None,
            shared_usage: None,
            total_committed: None,
        }
    }


    /// Sets the value of DedicatedUsage
    pub fn set_dedicated_usage(&mut self, value: u64) {
        self.dedicated_usage = Some(value);
    }

    /// Gets the value of DedicatedUsage
    pub fn get_dedicated_usage(&self) -> Option<&u64> {
        self.dedicated_usage.as_ref()
    }

    /// Sets the value of LocalUsage
    pub fn set_local_usage(&mut self, value: u64) {
        self.local_usage = Some(value);
    }

    /// Gets the value of LocalUsage
    pub fn get_local_usage(&self) -> Option<&u64> {
        self.local_usage.as_ref()
    }

    /// Sets the value of NonLocalUsage
    pub fn set_non_local_usage(&mut self, value: u64) {
        self.non_local_usage = Some(value);
    }

    /// Gets the value of NonLocalUsage
    pub fn get_non_local_usage(&self) -> Option<&u64> {
        self.non_local_usage.as_ref()
    }

    /// Sets the value of SharedUsage
    pub fn set_shared_usage(&mut self, value: u64) {
        self.shared_usage = Some(value);
    }

    /// Gets the value of SharedUsage
    pub fn get_shared_usage(&self) -> Option<&u64> {
        self.shared_usage.as_ref()
    }

    /// Sets the value of TotalCommitted
    pub fn set_total_committed(&mut self, value: u64) {
        self.total_committed = Some(value);
    }

    /// Gets the value of TotalCommitted
    pub fn get_total_committed(&self) -> Option<&u64> {
        self.total_committed.as_ref()
    }
}

