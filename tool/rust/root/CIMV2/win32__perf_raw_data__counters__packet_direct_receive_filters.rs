// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_PacketDirectReceiveFilters struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_PacketDirectReceiveFilters {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "BytesMatched")]
    pub bytes_matched: Option<u64>,

/// 
    #[serde(rename = "BytesMatchedPersec")]
    pub bytes_matched_persec: Option<u64>,

/// 
    #[serde(rename = "PacketsMatched")]
    pub packets_matched: Option<u64>,

/// 
    #[serde(rename = "PacketsMatchedPersec")]
    pub packets_matched_persec: Option<u64>,
}

impl Win32_PerfRawData_Counters_PacketDirectReceiveFilters {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            bytes_matched: None,
            bytes_matched_persec: None,
            packets_matched: None,
            packets_matched_persec: None,
        }
    }


    /// Sets the value of BytesMatched
    pub fn set_bytes_matched(&mut self, value: u64) {
        self.bytes_matched = Some(value);
    }

    /// Gets the value of BytesMatched
    pub fn get_bytes_matched(&self) -> Option<&u64> {
        self.bytes_matched.as_ref()
    }

    /// Sets the value of BytesMatchedPersec
    pub fn set_bytes_matched_persec(&mut self, value: u64) {
        self.bytes_matched_persec = Some(value);
    }

    /// Gets the value of BytesMatchedPersec
    pub fn get_bytes_matched_persec(&self) -> Option<&u64> {
        self.bytes_matched_persec.as_ref()
    }

    /// Sets the value of PacketsMatched
    pub fn set_packets_matched(&mut self, value: u64) {
        self.packets_matched = Some(value);
    }

    /// Gets the value of PacketsMatched
    pub fn get_packets_matched(&self) -> Option<&u64> {
        self.packets_matched.as_ref()
    }

    /// Sets the value of PacketsMatchedPersec
    pub fn set_packets_matched_persec(&mut self, value: u64) {
        self.packets_matched_persec = Some(value);
    }

    /// Gets the value of PacketsMatchedPersec
    pub fn get_packets_matched_persec(&self) -> Option<&u64> {
        self.packets_matched_persec.as_ref()
    }
}

