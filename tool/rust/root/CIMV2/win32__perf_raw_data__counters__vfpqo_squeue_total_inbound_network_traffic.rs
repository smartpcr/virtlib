// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_VFPQoSQueueTotalInboundNetworkTraffic struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_VFPQoSQueueTotalInboundNetworkTraffic {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "TotalInboundBytesDropped")]
    pub total_inbound_bytes_dropped: Option<u64>,

/// 
    #[serde(rename = "TotalInboundPacketsDropped")]
    pub total_inbound_packets_dropped: Option<u64>,
}

impl Win32_PerfRawData_Counters_VFPQoSQueueTotalInboundNetworkTraffic {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            total_inbound_bytes_dropped: None,
            total_inbound_packets_dropped: None,
        }
    }


    /// Sets the value of TotalInboundBytesDropped
    pub fn set_total_inbound_bytes_dropped(&mut self, value: u64) {
        self.total_inbound_bytes_dropped = Some(value);
    }

    /// Gets the value of TotalInboundBytesDropped
    pub fn get_total_inbound_bytes_dropped(&self) -> Option<&u64> {
        self.total_inbound_bytes_dropped.as_ref()
    }

    /// Sets the value of TotalInboundPacketsDropped
    pub fn set_total_inbound_packets_dropped(&mut self, value: u64) {
        self.total_inbound_packets_dropped = Some(value);
    }

    /// Gets the value of TotalInboundPacketsDropped
    pub fn get_total_inbound_packets_dropped(&self) -> Option<&u64> {
        self.total_inbound_packets_dropped.as_ref()
    }
}

