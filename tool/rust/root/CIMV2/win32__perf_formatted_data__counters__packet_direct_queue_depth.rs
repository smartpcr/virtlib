// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_PacketDirectQueueDepth struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_PacketDirectQueueDepth {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "AverageQueueDepth")]
    pub average_queue_depth: Option<u32>,

/// 
    #[serde(rename = "PercentAverageQueueUtilization")]
    pub percent_average_queue_utilization: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_PacketDirectQueueDepth {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            average_queue_depth: None,
            percent_average_queue_utilization: None,
        }
    }


    /// Sets the value of AverageQueueDepth
    pub fn set_average_queue_depth(&mut self, value: u32) {
        self.average_queue_depth = Some(value);
    }

    /// Gets the value of AverageQueueDepth
    pub fn get_average_queue_depth(&self) -> Option<&u32> {
        self.average_queue_depth.as_ref()
    }

    /// Sets the value of PercentAverageQueueUtilization
    pub fn set_percent_average_queue_utilization(&mut self, value: u32) {
        self.percent_average_queue_utilization = Some(value);
    }

    /// Gets the value of PercentAverageQueueUtilization
    pub fn get_percent_average_queue_utilization(&self) -> Option<&u32> {
        self.percent_average_queue_utilization.as_ref()
    }
}

