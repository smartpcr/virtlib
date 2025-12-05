// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_PacketDirectReceiveCounters struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_PacketDirectReceiveCounters {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "BytesReceived")]
    pub bytes_received: Option<u64>,

/// 
    #[serde(rename = "BytesReceivedPersec")]
    pub bytes_received_persec: Option<u64>,

/// 
    #[serde(rename = "PacketsDropped")]
    pub packets_dropped: Option<u64>,

/// 
    #[serde(rename = "PacketsDroppedPersec")]
    pub packets_dropped_persec: Option<u64>,

/// 
    #[serde(rename = "PacketsReceived")]
    pub packets_received: Option<u64>,

/// 
    #[serde(rename = "PacketsReceivedPersec")]
    pub packets_received_persec: Option<u64>,
}

impl Win32_PerfFormattedData_Counters_PacketDirectReceiveCounters {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            bytes_received: None,
            bytes_received_persec: None,
            packets_dropped: None,
            packets_dropped_persec: None,
            packets_received: None,
            packets_received_persec: None,
        }
    }


    /// Sets the value of BytesReceived
    pub fn set_bytes_received(&mut self, value: u64) {
        self.bytes_received = Some(value);
    }

    /// Gets the value of BytesReceived
    pub fn get_bytes_received(&self) -> Option<&u64> {
        self.bytes_received.as_ref()
    }

    /// Sets the value of BytesReceivedPersec
    pub fn set_bytes_received_persec(&mut self, value: u64) {
        self.bytes_received_persec = Some(value);
    }

    /// Gets the value of BytesReceivedPersec
    pub fn get_bytes_received_persec(&self) -> Option<&u64> {
        self.bytes_received_persec.as_ref()
    }

    /// Sets the value of PacketsDropped
    pub fn set_packets_dropped(&mut self, value: u64) {
        self.packets_dropped = Some(value);
    }

    /// Gets the value of PacketsDropped
    pub fn get_packets_dropped(&self) -> Option<&u64> {
        self.packets_dropped.as_ref()
    }

    /// Sets the value of PacketsDroppedPersec
    pub fn set_packets_dropped_persec(&mut self, value: u64) {
        self.packets_dropped_persec = Some(value);
    }

    /// Gets the value of PacketsDroppedPersec
    pub fn get_packets_dropped_persec(&self) -> Option<&u64> {
        self.packets_dropped_persec.as_ref()
    }

    /// Sets the value of PacketsReceived
    pub fn set_packets_received(&mut self, value: u64) {
        self.packets_received = Some(value);
    }

    /// Gets the value of PacketsReceived
    pub fn get_packets_received(&self) -> Option<&u64> {
        self.packets_received.as_ref()
    }

    /// Sets the value of PacketsReceivedPersec
    pub fn set_packets_received_persec(&mut self, value: u64) {
        self.packets_received_persec = Some(value);
    }

    /// Gets the value of PacketsReceivedPersec
    pub fn get_packets_received_persec(&self) -> Option<&u64> {
        self.packets_received_persec.as_ref()
    }
}

