// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Tcpip_NetworkInterface struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Tcpip_NetworkInterface {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "BytesReceivedPersec")]
    pub bytes_received_persec: Option<u64>,

/// 
    #[serde(rename = "BytesSentPersec")]
    pub bytes_sent_persec: Option<u64>,

/// 
    #[serde(rename = "BytesTotalPersec")]
    pub bytes_total_persec: Option<u64>,

/// 
    #[serde(rename = "CurrentBandwidth")]
    pub current_bandwidth: Option<u64>,

/// 
    #[serde(rename = "OffloadedConnections")]
    pub offloaded_connections: Option<u64>,

/// 
    #[serde(rename = "OutputQueueLength")]
    pub output_queue_length: Option<u64>,

/// 
    #[serde(rename = "PacketsOutboundDiscarded")]
    pub packets_outbound_discarded: Option<u64>,

/// 
    #[serde(rename = "PacketsOutboundErrors")]
    pub packets_outbound_errors: Option<u64>,

/// 
    #[serde(rename = "PacketsPersec")]
    pub packets_persec: Option<u64>,

/// 
    #[serde(rename = "PacketsReceivedDiscarded")]
    pub packets_received_discarded: Option<u64>,

/// 
    #[serde(rename = "PacketsReceivedErrors")]
    pub packets_received_errors: Option<u64>,

/// 
    #[serde(rename = "PacketsReceivedNonUnicastPersec")]
    pub packets_received_non_unicast_persec: Option<u64>,

/// 
    #[serde(rename = "PacketsReceivedPersec")]
    pub packets_received_persec: Option<u64>,

/// 
    #[serde(rename = "PacketsReceivedUnicastPersec")]
    pub packets_received_unicast_persec: Option<u64>,

/// 
    #[serde(rename = "PacketsReceivedUnknown")]
    pub packets_received_unknown: Option<u64>,

/// 
    #[serde(rename = "PacketsSentNonUnicastPersec")]
    pub packets_sent_non_unicast_persec: Option<u64>,

/// 
    #[serde(rename = "PacketsSentPersec")]
    pub packets_sent_persec: Option<u64>,

/// 
    #[serde(rename = "PacketsSentUnicastPersec")]
    pub packets_sent_unicast_persec: Option<u64>,

/// 
    #[serde(rename = "TCPActiveRSCConnections")]
    pub tcpactive_rscconnections: Option<u64>,

/// 
    #[serde(rename = "TCPRSCAveragePacketSize")]
    pub tcprscaverage_packet_size: Option<u64>,

/// 
    #[serde(rename = "TCPRSCCoalescedPacketsPersec")]
    pub tcprsccoalesced_packets_persec: Option<u64>,

/// 
    #[serde(rename = "TCPRSCExceptionsPersec")]
    pub tcprscexceptions_persec: Option<u64>,
}

impl Win32_PerfRawData_Tcpip_NetworkInterface {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            bytes_received_persec: None,
            bytes_sent_persec: None,
            bytes_total_persec: None,
            current_bandwidth: None,
            offloaded_connections: None,
            output_queue_length: None,
            packets_outbound_discarded: None,
            packets_outbound_errors: None,
            packets_persec: None,
            packets_received_discarded: None,
            packets_received_errors: None,
            packets_received_non_unicast_persec: None,
            packets_received_persec: None,
            packets_received_unicast_persec: None,
            packets_received_unknown: None,
            packets_sent_non_unicast_persec: None,
            packets_sent_persec: None,
            packets_sent_unicast_persec: None,
            tcpactive_rscconnections: None,
            tcprscaverage_packet_size: None,
            tcprsccoalesced_packets_persec: None,
            tcprscexceptions_persec: None,
        }
    }


    /// Sets the value of BytesReceivedPersec
    pub fn set_bytes_received_persec(&mut self, value: u64) {
        self.bytes_received_persec = Some(value);
    }

    /// Gets the value of BytesReceivedPersec
    pub fn get_bytes_received_persec(&self) -> Option<&u64> {
        self.bytes_received_persec.as_ref()
    }

    /// Sets the value of BytesSentPersec
    pub fn set_bytes_sent_persec(&mut self, value: u64) {
        self.bytes_sent_persec = Some(value);
    }

    /// Gets the value of BytesSentPersec
    pub fn get_bytes_sent_persec(&self) -> Option<&u64> {
        self.bytes_sent_persec.as_ref()
    }

    /// Sets the value of BytesTotalPersec
    pub fn set_bytes_total_persec(&mut self, value: u64) {
        self.bytes_total_persec = Some(value);
    }

    /// Gets the value of BytesTotalPersec
    pub fn get_bytes_total_persec(&self) -> Option<&u64> {
        self.bytes_total_persec.as_ref()
    }

    /// Sets the value of CurrentBandwidth
    pub fn set_current_bandwidth(&mut self, value: u64) {
        self.current_bandwidth = Some(value);
    }

    /// Gets the value of CurrentBandwidth
    pub fn get_current_bandwidth(&self) -> Option<&u64> {
        self.current_bandwidth.as_ref()
    }

    /// Sets the value of OffloadedConnections
    pub fn set_offloaded_connections(&mut self, value: u64) {
        self.offloaded_connections = Some(value);
    }

    /// Gets the value of OffloadedConnections
    pub fn get_offloaded_connections(&self) -> Option<&u64> {
        self.offloaded_connections.as_ref()
    }

    /// Sets the value of OutputQueueLength
    pub fn set_output_queue_length(&mut self, value: u64) {
        self.output_queue_length = Some(value);
    }

    /// Gets the value of OutputQueueLength
    pub fn get_output_queue_length(&self) -> Option<&u64> {
        self.output_queue_length.as_ref()
    }

    /// Sets the value of PacketsOutboundDiscarded
    pub fn set_packets_outbound_discarded(&mut self, value: u64) {
        self.packets_outbound_discarded = Some(value);
    }

    /// Gets the value of PacketsOutboundDiscarded
    pub fn get_packets_outbound_discarded(&self) -> Option<&u64> {
        self.packets_outbound_discarded.as_ref()
    }

    /// Sets the value of PacketsOutboundErrors
    pub fn set_packets_outbound_errors(&mut self, value: u64) {
        self.packets_outbound_errors = Some(value);
    }

    /// Gets the value of PacketsOutboundErrors
    pub fn get_packets_outbound_errors(&self) -> Option<&u64> {
        self.packets_outbound_errors.as_ref()
    }

    /// Sets the value of PacketsPersec
    pub fn set_packets_persec(&mut self, value: u64) {
        self.packets_persec = Some(value);
    }

    /// Gets the value of PacketsPersec
    pub fn get_packets_persec(&self) -> Option<&u64> {
        self.packets_persec.as_ref()
    }

    /// Sets the value of PacketsReceivedDiscarded
    pub fn set_packets_received_discarded(&mut self, value: u64) {
        self.packets_received_discarded = Some(value);
    }

    /// Gets the value of PacketsReceivedDiscarded
    pub fn get_packets_received_discarded(&self) -> Option<&u64> {
        self.packets_received_discarded.as_ref()
    }

    /// Sets the value of PacketsReceivedErrors
    pub fn set_packets_received_errors(&mut self, value: u64) {
        self.packets_received_errors = Some(value);
    }

    /// Gets the value of PacketsReceivedErrors
    pub fn get_packets_received_errors(&self) -> Option<&u64> {
        self.packets_received_errors.as_ref()
    }

    /// Sets the value of PacketsReceivedNonUnicastPersec
    pub fn set_packets_received_non_unicast_persec(&mut self, value: u64) {
        self.packets_received_non_unicast_persec = Some(value);
    }

    /// Gets the value of PacketsReceivedNonUnicastPersec
    pub fn get_packets_received_non_unicast_persec(&self) -> Option<&u64> {
        self.packets_received_non_unicast_persec.as_ref()
    }

    /// Sets the value of PacketsReceivedPersec
    pub fn set_packets_received_persec(&mut self, value: u64) {
        self.packets_received_persec = Some(value);
    }

    /// Gets the value of PacketsReceivedPersec
    pub fn get_packets_received_persec(&self) -> Option<&u64> {
        self.packets_received_persec.as_ref()
    }

    /// Sets the value of PacketsReceivedUnicastPersec
    pub fn set_packets_received_unicast_persec(&mut self, value: u64) {
        self.packets_received_unicast_persec = Some(value);
    }

    /// Gets the value of PacketsReceivedUnicastPersec
    pub fn get_packets_received_unicast_persec(&self) -> Option<&u64> {
        self.packets_received_unicast_persec.as_ref()
    }

    /// Sets the value of PacketsReceivedUnknown
    pub fn set_packets_received_unknown(&mut self, value: u64) {
        self.packets_received_unknown = Some(value);
    }

    /// Gets the value of PacketsReceivedUnknown
    pub fn get_packets_received_unknown(&self) -> Option<&u64> {
        self.packets_received_unknown.as_ref()
    }

    /// Sets the value of PacketsSentNonUnicastPersec
    pub fn set_packets_sent_non_unicast_persec(&mut self, value: u64) {
        self.packets_sent_non_unicast_persec = Some(value);
    }

    /// Gets the value of PacketsSentNonUnicastPersec
    pub fn get_packets_sent_non_unicast_persec(&self) -> Option<&u64> {
        self.packets_sent_non_unicast_persec.as_ref()
    }

    /// Sets the value of PacketsSentPersec
    pub fn set_packets_sent_persec(&mut self, value: u64) {
        self.packets_sent_persec = Some(value);
    }

    /// Gets the value of PacketsSentPersec
    pub fn get_packets_sent_persec(&self) -> Option<&u64> {
        self.packets_sent_persec.as_ref()
    }

    /// Sets the value of PacketsSentUnicastPersec
    pub fn set_packets_sent_unicast_persec(&mut self, value: u64) {
        self.packets_sent_unicast_persec = Some(value);
    }

    /// Gets the value of PacketsSentUnicastPersec
    pub fn get_packets_sent_unicast_persec(&self) -> Option<&u64> {
        self.packets_sent_unicast_persec.as_ref()
    }

    /// Sets the value of TCPActiveRSCConnections
    pub fn set_tcpactive_rscconnections(&mut self, value: u64) {
        self.tcpactive_rscconnections = Some(value);
    }

    /// Gets the value of TCPActiveRSCConnections
    pub fn get_tcpactive_rscconnections(&self) -> Option<&u64> {
        self.tcpactive_rscconnections.as_ref()
    }

    /// Sets the value of TCPRSCAveragePacketSize
    pub fn set_tcprscaverage_packet_size(&mut self, value: u64) {
        self.tcprscaverage_packet_size = Some(value);
    }

    /// Gets the value of TCPRSCAveragePacketSize
    pub fn get_tcprscaverage_packet_size(&self) -> Option<&u64> {
        self.tcprscaverage_packet_size.as_ref()
    }

    /// Sets the value of TCPRSCCoalescedPacketsPersec
    pub fn set_tcprsccoalesced_packets_persec(&mut self, value: u64) {
        self.tcprsccoalesced_packets_persec = Some(value);
    }

    /// Gets the value of TCPRSCCoalescedPacketsPersec
    pub fn get_tcprsccoalesced_packets_persec(&self) -> Option<&u64> {
        self.tcprsccoalesced_packets_persec.as_ref()
    }

    /// Sets the value of TCPRSCExceptionsPersec
    pub fn set_tcprscexceptions_persec(&mut self, value: u64) {
        self.tcprscexceptions_persec = Some(value);
    }

    /// Gets the value of TCPRSCExceptionsPersec
    pub fn get_tcprscexceptions_persec(&self) -> Option<&u64> {
        self.tcprscexceptions_persec.as_ref()
    }
}

