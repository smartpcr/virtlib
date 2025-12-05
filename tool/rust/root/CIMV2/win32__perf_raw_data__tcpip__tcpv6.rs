// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Tcpip_TCPv6 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Tcpip_TCPv6 {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "ConnectionFailures")]
    pub connection_failures: Option<u32>,

/// 
    #[serde(rename = "ConnectionsActive")]
    pub connections_active: Option<u32>,

/// 
    #[serde(rename = "ConnectionsEstablished")]
    pub connections_established: Option<u32>,

/// 
    #[serde(rename = "ConnectionsPassive")]
    pub connections_passive: Option<u32>,

/// 
    #[serde(rename = "ConnectionsReset")]
    pub connections_reset: Option<u32>,

/// 
    #[serde(rename = "SegmentsPersec")]
    pub segments_persec: Option<u32>,

/// 
    #[serde(rename = "SegmentsReceivedPersec")]
    pub segments_received_persec: Option<u32>,

/// 
    #[serde(rename = "SegmentsRetransmittedPersec")]
    pub segments_retransmitted_persec: Option<u32>,

/// 
    #[serde(rename = "SegmentsSentPersec")]
    pub segments_sent_persec: Option<u32>,
}

impl Win32_PerfRawData_Tcpip_TCPv6 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            connection_failures: None,
            connections_active: None,
            connections_established: None,
            connections_passive: None,
            connections_reset: None,
            segments_persec: None,
            segments_received_persec: None,
            segments_retransmitted_persec: None,
            segments_sent_persec: None,
        }
    }


    /// Sets the value of ConnectionFailures
    pub fn set_connection_failures(&mut self, value: u32) {
        self.connection_failures = Some(value);
    }

    /// Gets the value of ConnectionFailures
    pub fn get_connection_failures(&self) -> Option<&u32> {
        self.connection_failures.as_ref()
    }

    /// Sets the value of ConnectionsActive
    pub fn set_connections_active(&mut self, value: u32) {
        self.connections_active = Some(value);
    }

    /// Gets the value of ConnectionsActive
    pub fn get_connections_active(&self) -> Option<&u32> {
        self.connections_active.as_ref()
    }

    /// Sets the value of ConnectionsEstablished
    pub fn set_connections_established(&mut self, value: u32) {
        self.connections_established = Some(value);
    }

    /// Gets the value of ConnectionsEstablished
    pub fn get_connections_established(&self) -> Option<&u32> {
        self.connections_established.as_ref()
    }

    /// Sets the value of ConnectionsPassive
    pub fn set_connections_passive(&mut self, value: u32) {
        self.connections_passive = Some(value);
    }

    /// Gets the value of ConnectionsPassive
    pub fn get_connections_passive(&self) -> Option<&u32> {
        self.connections_passive.as_ref()
    }

    /// Sets the value of ConnectionsReset
    pub fn set_connections_reset(&mut self, value: u32) {
        self.connections_reset = Some(value);
    }

    /// Gets the value of ConnectionsReset
    pub fn get_connections_reset(&self) -> Option<&u32> {
        self.connections_reset.as_ref()
    }

    /// Sets the value of SegmentsPersec
    pub fn set_segments_persec(&mut self, value: u32) {
        self.segments_persec = Some(value);
    }

    /// Gets the value of SegmentsPersec
    pub fn get_segments_persec(&self) -> Option<&u32> {
        self.segments_persec.as_ref()
    }

    /// Sets the value of SegmentsReceivedPersec
    pub fn set_segments_received_persec(&mut self, value: u32) {
        self.segments_received_persec = Some(value);
    }

    /// Gets the value of SegmentsReceivedPersec
    pub fn get_segments_received_persec(&self) -> Option<&u32> {
        self.segments_received_persec.as_ref()
    }

    /// Sets the value of SegmentsRetransmittedPersec
    pub fn set_segments_retransmitted_persec(&mut self, value: u32) {
        self.segments_retransmitted_persec = Some(value);
    }

    /// Gets the value of SegmentsRetransmittedPersec
    pub fn get_segments_retransmitted_persec(&self) -> Option<&u32> {
        self.segments_retransmitted_persec.as_ref()
    }

    /// Sets the value of SegmentsSentPersec
    pub fn set_segments_sent_persec(&mut self, value: u32) {
        self.segments_sent_persec = Some(value);
    }

    /// Gets the value of SegmentsSentPersec
    pub fn get_segments_sent_persec(&self) -> Option<&u32> {
        self.segments_sent_persec.as_ref()
    }
}

