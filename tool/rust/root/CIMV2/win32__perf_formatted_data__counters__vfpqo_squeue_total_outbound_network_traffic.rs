// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_VFPQoSQueueTotalOutboundNetworkTraffic struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_VFPQoSQueueTotalOutboundNetworkTraffic {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "TotalOutboundBytesDropped")]
    pub total_outbound_bytes_dropped: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundPacketsDropped")]
    pub total_outbound_packets_dropped: Option<u64>,
}

impl Win32_PerfFormattedData_Counters_VFPQoSQueueTotalOutboundNetworkTraffic {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            total_outbound_bytes_dropped: None,
            total_outbound_packets_dropped: None,
        }
    }


    /// Sets the value of TotalOutboundBytesDropped
    pub fn set_total_outbound_bytes_dropped(&mut self, value: u64) {
        self.total_outbound_bytes_dropped = Some(value);
    }

    /// Gets the value of TotalOutboundBytesDropped
    pub fn get_total_outbound_bytes_dropped(&self) -> Option<&u64> {
        self.total_outbound_bytes_dropped.as_ref()
    }

    /// Sets the value of TotalOutboundPacketsDropped
    pub fn set_total_outbound_packets_dropped(&mut self, value: u64) {
        self.total_outbound_packets_dropped = Some(value);
    }

    /// Gets the value of TotalOutboundPacketsDropped
    pub fn get_total_outbound_packets_dropped(&self) -> Option<&u64> {
        self.total_outbound_packets_dropped.as_ref()
    }
}

