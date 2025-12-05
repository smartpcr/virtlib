// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_PacketDirectTransmitCounters struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_PacketDirectTransmitCounters {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "BytesTransmitted")]
    pub bytes_transmitted: Option<u64>,

/// 
    #[serde(rename = "BytesTransmittedPersec")]
    pub bytes_transmitted_persec: Option<u64>,

/// 
    #[serde(rename = "PacketsTransmitted")]
    pub packets_transmitted: Option<u64>,

/// 
    #[serde(rename = "PacketsTransmittedPersec")]
    pub packets_transmitted_persec: Option<u64>,
}

impl Win32_PerfFormattedData_Counters_PacketDirectTransmitCounters {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            bytes_transmitted: None,
            bytes_transmitted_persec: None,
            packets_transmitted: None,
            packets_transmitted_persec: None,
        }
    }


    /// Sets the value of BytesTransmitted
    pub fn set_bytes_transmitted(&mut self, value: u64) {
        self.bytes_transmitted = Some(value);
    }

    /// Gets the value of BytesTransmitted
    pub fn get_bytes_transmitted(&self) -> Option<&u64> {
        self.bytes_transmitted.as_ref()
    }

    /// Sets the value of BytesTransmittedPersec
    pub fn set_bytes_transmitted_persec(&mut self, value: u64) {
        self.bytes_transmitted_persec = Some(value);
    }

    /// Gets the value of BytesTransmittedPersec
    pub fn get_bytes_transmitted_persec(&self) -> Option<&u64> {
        self.bytes_transmitted_persec.as_ref()
    }

    /// Sets the value of PacketsTransmitted
    pub fn set_packets_transmitted(&mut self, value: u64) {
        self.packets_transmitted = Some(value);
    }

    /// Gets the value of PacketsTransmitted
    pub fn get_packets_transmitted(&self) -> Option<&u64> {
        self.packets_transmitted.as_ref()
    }

    /// Sets the value of PacketsTransmittedPersec
    pub fn set_packets_transmitted_persec(&mut self, value: u64) {
        self.packets_transmitted_persec = Some(value);
    }

    /// Gets the value of PacketsTransmittedPersec
    pub fn get_packets_transmitted_persec(&self) -> Option<&u64> {
        self.packets_transmitted_persec.as_ref()
    }
}

