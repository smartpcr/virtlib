// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_GPUPerformanceCounters_GPUEngine struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_GPUPerformanceCounters_GPUEngine {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "RunningTime")]
    pub running_time: Option<u64>,

/// 
    #[serde(rename = "UtilizationPercentage")]
    pub utilization_percentage: Option<u64>,
}

impl Win32_PerfFormattedData_GPUPerformanceCounters_GPUEngine {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            running_time: None,
            utilization_percentage: None,
        }
    }


    /// Sets the value of RunningTime
    pub fn set_running_time(&mut self, value: u64) {
        self.running_time = Some(value);
    }

    /// Gets the value of RunningTime
    pub fn get_running_time(&self) -> Option<&u64> {
        self.running_time.as_ref()
    }

    /// Sets the value of UtilizationPercentage
    pub fn set_utilization_percentage(&mut self, value: u64) {
        self.utilization_percentage = Some(value);
    }

    /// Gets the value of UtilizationPercentage
    pub fn get_utilization_percentage(&self) -> Option<&u64> {
        self.utilization_percentage.as_ref()
    }
}

