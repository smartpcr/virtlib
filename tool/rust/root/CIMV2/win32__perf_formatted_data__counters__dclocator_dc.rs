// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_DCLocatorDC struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_DCLocatorDC {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "PingsActiveMailslotPings")]
    pub pings_active_mailslot_pings: Option<u32>,

/// 
    #[serde(rename = "PingsActiveUDPLDAPPings")]
    pub pings_active_udpldappings: Option<u32>,

/// 
    #[serde(rename = "PingsAverageMailslotPingLatencysecs")]
    pub pings_average_mailslot_ping_latencysecs: Option<u32>,

/// 
    #[serde(rename = "PingsAverageUDPLDAPPingLatencysecs")]
    pub pings_average_udpldapping_latencysecs: Option<u32>,

/// 
    #[serde(rename = "PingsMailslotPingsReceivedPersec")]
    pub pings_mailslot_pings_received_persec: Option<u32>,

/// 
    #[serde(rename = "PingsUDPLDAPPingsReceivedPersec")]
    pub pings_udpldappings_received_persec: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_DCLocatorDC {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            pings_active_mailslot_pings: None,
            pings_active_udpldappings: None,
            pings_average_mailslot_ping_latencysecs: None,
            pings_average_udpldapping_latencysecs: None,
            pings_mailslot_pings_received_persec: None,
            pings_udpldappings_received_persec: None,
        }
    }


    /// Sets the value of PingsActiveMailslotPings
    pub fn set_pings_active_mailslot_pings(&mut self, value: u32) {
        self.pings_active_mailslot_pings = Some(value);
    }

    /// Gets the value of PingsActiveMailslotPings
    pub fn get_pings_active_mailslot_pings(&self) -> Option<&u32> {
        self.pings_active_mailslot_pings.as_ref()
    }

    /// Sets the value of PingsActiveUDPLDAPPings
    pub fn set_pings_active_udpldappings(&mut self, value: u32) {
        self.pings_active_udpldappings = Some(value);
    }

    /// Gets the value of PingsActiveUDPLDAPPings
    pub fn get_pings_active_udpldappings(&self) -> Option<&u32> {
        self.pings_active_udpldappings.as_ref()
    }

    /// Sets the value of PingsAverageMailslotPingLatencysecs
    pub fn set_pings_average_mailslot_ping_latencysecs(&mut self, value: u32) {
        self.pings_average_mailslot_ping_latencysecs = Some(value);
    }

    /// Gets the value of PingsAverageMailslotPingLatencysecs
    pub fn get_pings_average_mailslot_ping_latencysecs(&self) -> Option<&u32> {
        self.pings_average_mailslot_ping_latencysecs.as_ref()
    }

    /// Sets the value of PingsAverageUDPLDAPPingLatencysecs
    pub fn set_pings_average_udpldapping_latencysecs(&mut self, value: u32) {
        self.pings_average_udpldapping_latencysecs = Some(value);
    }

    /// Gets the value of PingsAverageUDPLDAPPingLatencysecs
    pub fn get_pings_average_udpldapping_latencysecs(&self) -> Option<&u32> {
        self.pings_average_udpldapping_latencysecs.as_ref()
    }

    /// Sets the value of PingsMailslotPingsReceivedPersec
    pub fn set_pings_mailslot_pings_received_persec(&mut self, value: u32) {
        self.pings_mailslot_pings_received_persec = Some(value);
    }

    /// Gets the value of PingsMailslotPingsReceivedPersec
    pub fn get_pings_mailslot_pings_received_persec(&self) -> Option<&u32> {
        self.pings_mailslot_pings_received_persec.as_ref()
    }

    /// Sets the value of PingsUDPLDAPPingsReceivedPersec
    pub fn set_pings_udpldappings_received_persec(&mut self, value: u32) {
        self.pings_udpldappings_received_persec = Some(value);
    }

    /// Gets the value of PingsUDPLDAPPingsReceivedPersec
    pub fn get_pings_udpldappings_received_persec(&self) -> Option<&u32> {
        self.pings_udpldappings_received_persec.as_ref()
    }
}

