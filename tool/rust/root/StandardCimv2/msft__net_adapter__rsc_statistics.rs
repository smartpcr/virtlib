// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapter_RscStatistics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapter_RscStatistics {

/// 
    #[serde(rename = "CoalescedBytes")]
    pub coalesced_bytes: Option<u64>,

/// 
    #[serde(rename = "CoalescedPackets")]
    pub coalesced_packets: Option<u64>,

/// 
    #[serde(rename = "CoalescingEvents")]
    pub coalescing_events: Option<u64>,

/// 
    #[serde(rename = "CoalescingExceptions")]
    pub coalescing_exceptions: Option<u64>,
}

impl MSFT_NetAdapter_RscStatistics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            coalesced_bytes: None,
            coalesced_packets: None,
            coalescing_events: None,
            coalescing_exceptions: None,
        }
    }


    /// Sets the value of CoalescedBytes
    pub fn set_coalesced_bytes(&mut self, value: u64) {
        self.coalesced_bytes = Some(value);
    }

    /// Gets the value of CoalescedBytes
    pub fn get_coalesced_bytes(&self) -> Option<&u64> {
        self.coalesced_bytes.as_ref()
    }

    /// Sets the value of CoalescedPackets
    pub fn set_coalesced_packets(&mut self, value: u64) {
        self.coalesced_packets = Some(value);
    }

    /// Gets the value of CoalescedPackets
    pub fn get_coalesced_packets(&self) -> Option<&u64> {
        self.coalesced_packets.as_ref()
    }

    /// Sets the value of CoalescingEvents
    pub fn set_coalescing_events(&mut self, value: u64) {
        self.coalescing_events = Some(value);
    }

    /// Gets the value of CoalescingEvents
    pub fn get_coalescing_events(&self) -> Option<&u64> {
        self.coalescing_events.as_ref()
    }

    /// Sets the value of CoalescingExceptions
    pub fn set_coalescing_exceptions(&mut self, value: u64) {
        self.coalescing_exceptions = Some(value);
    }

    /// Gets the value of CoalescingExceptions
    pub fn get_coalescing_exceptions(&self) -> Option<&u64> {
        self.coalescing_exceptions.as_ref()
    }
}

