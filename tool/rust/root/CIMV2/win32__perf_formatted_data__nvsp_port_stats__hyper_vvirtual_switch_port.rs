// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_NvspPortStats_HyperVVirtualSwitchPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_NvspPortStats_HyperVVirtualSwitchPort {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "BroadcastPacketsReceivedPersec")]
    pub broadcast_packets_received_persec: Option<u64>,

/// 
    #[serde(rename = "BroadcastPacketsSentPersec")]
    pub broadcast_packets_sent_persec: Option<u64>,

/// 
    #[serde(rename = "BytesPersec")]
    pub bytes_persec: Option<u64>,

/// 
    #[serde(rename = "BytesReceivedPersec")]
    pub bytes_received_persec: Option<u64>,

/// 
    #[serde(rename = "BytesSentPersec")]
    pub bytes_sent_persec: Option<u64>,

/// 
    #[serde(rename = "DirectedPacketsReceivedPersec")]
    pub directed_packets_received_persec: Option<u64>,

/// 
    #[serde(rename = "DirectedPacketsSentPersec")]
    pub directed_packets_sent_persec: Option<u64>,

/// 
    #[serde(rename = "DroppedPacketsIncomingPersec")]
    pub dropped_packets_incoming_persec: Option<u64>,

/// 
    #[serde(rename = "DroppedPacketsOutgoingPersec")]
    pub dropped_packets_outgoing_persec: Option<u64>,

/// 
    #[serde(rename = "ExtensionsDroppedPacketsIncomingPersec")]
    pub extensions_dropped_packets_incoming_persec: Option<u64>,

/// 
    #[serde(rename = "ExtensionsDroppedPacketsOutgoingPersec")]
    pub extensions_dropped_packets_outgoing_persec: Option<u64>,

/// 
    #[serde(rename = "IPsecoffloadBytesReceivePersec")]
    pub ipsecoffload_bytes_receive_persec: Option<u64>,

/// 
    #[serde(rename = "IPsecoffloadBytesSentPersec")]
    pub ipsecoffload_bytes_sent_persec: Option<u64>,

/// 
    #[serde(rename = "IPsecSAsOffloaded")]
    pub ipsec_sas_offloaded: Option<u64>,

/// 
    #[serde(rename = "MulticastPacketsReceivedPersec")]
    pub multicast_packets_received_persec: Option<u64>,

/// 
    #[serde(rename = "MulticastPacketsSentPersec")]
    pub multicast_packets_sent_persec: Option<u64>,

/// 
    #[serde(rename = "PacketsPersec")]
    pub packets_persec: Option<u64>,

/// 
    #[serde(rename = "PacketsReceivedPersec")]
    pub packets_received_persec: Option<u64>,

/// 
    #[serde(rename = "PacketsSentPersec")]
    pub packets_sent_persec: Option<u64>,

/// 
    #[serde(rename = "UnhashedPacketsReceivedPersec")]
    pub unhashed_packets_received_persec: Option<u64>,

/// 
    #[serde(rename = "UnhashedPacketsSendCompletedPersec")]
    pub unhashed_packets_send_completed_persec: Option<u64>,
}

impl Win32_PerfFormattedData_NvspPortStats_HyperVVirtualSwitchPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            broadcast_packets_received_persec: None,
            broadcast_packets_sent_persec: None,
            bytes_persec: None,
            bytes_received_persec: None,
            bytes_sent_persec: None,
            directed_packets_received_persec: None,
            directed_packets_sent_persec: None,
            dropped_packets_incoming_persec: None,
            dropped_packets_outgoing_persec: None,
            extensions_dropped_packets_incoming_persec: None,
            extensions_dropped_packets_outgoing_persec: None,
            ipsecoffload_bytes_receive_persec: None,
            ipsecoffload_bytes_sent_persec: None,
            ipsec_sas_offloaded: None,
            multicast_packets_received_persec: None,
            multicast_packets_sent_persec: None,
            packets_persec: None,
            packets_received_persec: None,
            packets_sent_persec: None,
            unhashed_packets_received_persec: None,
            unhashed_packets_send_completed_persec: None,
        }
    }


    /// Sets the value of BroadcastPacketsReceivedPersec
    pub fn set_broadcast_packets_received_persec(&mut self, value: u64) {
        self.broadcast_packets_received_persec = Some(value);
    }

    /// Gets the value of BroadcastPacketsReceivedPersec
    pub fn get_broadcast_packets_received_persec(&self) -> Option<&u64> {
        self.broadcast_packets_received_persec.as_ref()
    }

    /// Sets the value of BroadcastPacketsSentPersec
    pub fn set_broadcast_packets_sent_persec(&mut self, value: u64) {
        self.broadcast_packets_sent_persec = Some(value);
    }

    /// Gets the value of BroadcastPacketsSentPersec
    pub fn get_broadcast_packets_sent_persec(&self) -> Option<&u64> {
        self.broadcast_packets_sent_persec.as_ref()
    }

    /// Sets the value of BytesPersec
    pub fn set_bytes_persec(&mut self, value: u64) {
        self.bytes_persec = Some(value);
    }

    /// Gets the value of BytesPersec
    pub fn get_bytes_persec(&self) -> Option<&u64> {
        self.bytes_persec.as_ref()
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

    /// Sets the value of DirectedPacketsReceivedPersec
    pub fn set_directed_packets_received_persec(&mut self, value: u64) {
        self.directed_packets_received_persec = Some(value);
    }

    /// Gets the value of DirectedPacketsReceivedPersec
    pub fn get_directed_packets_received_persec(&self) -> Option<&u64> {
        self.directed_packets_received_persec.as_ref()
    }

    /// Sets the value of DirectedPacketsSentPersec
    pub fn set_directed_packets_sent_persec(&mut self, value: u64) {
        self.directed_packets_sent_persec = Some(value);
    }

    /// Gets the value of DirectedPacketsSentPersec
    pub fn get_directed_packets_sent_persec(&self) -> Option<&u64> {
        self.directed_packets_sent_persec.as_ref()
    }

    /// Sets the value of DroppedPacketsIncomingPersec
    pub fn set_dropped_packets_incoming_persec(&mut self, value: u64) {
        self.dropped_packets_incoming_persec = Some(value);
    }

    /// Gets the value of DroppedPacketsIncomingPersec
    pub fn get_dropped_packets_incoming_persec(&self) -> Option<&u64> {
        self.dropped_packets_incoming_persec.as_ref()
    }

    /// Sets the value of DroppedPacketsOutgoingPersec
    pub fn set_dropped_packets_outgoing_persec(&mut self, value: u64) {
        self.dropped_packets_outgoing_persec = Some(value);
    }

    /// Gets the value of DroppedPacketsOutgoingPersec
    pub fn get_dropped_packets_outgoing_persec(&self) -> Option<&u64> {
        self.dropped_packets_outgoing_persec.as_ref()
    }

    /// Sets the value of ExtensionsDroppedPacketsIncomingPersec
    pub fn set_extensions_dropped_packets_incoming_persec(&mut self, value: u64) {
        self.extensions_dropped_packets_incoming_persec = Some(value);
    }

    /// Gets the value of ExtensionsDroppedPacketsIncomingPersec
    pub fn get_extensions_dropped_packets_incoming_persec(&self) -> Option<&u64> {
        self.extensions_dropped_packets_incoming_persec.as_ref()
    }

    /// Sets the value of ExtensionsDroppedPacketsOutgoingPersec
    pub fn set_extensions_dropped_packets_outgoing_persec(&mut self, value: u64) {
        self.extensions_dropped_packets_outgoing_persec = Some(value);
    }

    /// Gets the value of ExtensionsDroppedPacketsOutgoingPersec
    pub fn get_extensions_dropped_packets_outgoing_persec(&self) -> Option<&u64> {
        self.extensions_dropped_packets_outgoing_persec.as_ref()
    }

    /// Sets the value of IPsecoffloadBytesReceivePersec
    pub fn set_ipsecoffload_bytes_receive_persec(&mut self, value: u64) {
        self.ipsecoffload_bytes_receive_persec = Some(value);
    }

    /// Gets the value of IPsecoffloadBytesReceivePersec
    pub fn get_ipsecoffload_bytes_receive_persec(&self) -> Option<&u64> {
        self.ipsecoffload_bytes_receive_persec.as_ref()
    }

    /// Sets the value of IPsecoffloadBytesSentPersec
    pub fn set_ipsecoffload_bytes_sent_persec(&mut self, value: u64) {
        self.ipsecoffload_bytes_sent_persec = Some(value);
    }

    /// Gets the value of IPsecoffloadBytesSentPersec
    pub fn get_ipsecoffload_bytes_sent_persec(&self) -> Option<&u64> {
        self.ipsecoffload_bytes_sent_persec.as_ref()
    }

    /// Sets the value of IPsecSAsOffloaded
    pub fn set_ipsec_sas_offloaded(&mut self, value: u64) {
        self.ipsec_sas_offloaded = Some(value);
    }

    /// Gets the value of IPsecSAsOffloaded
    pub fn get_ipsec_sas_offloaded(&self) -> Option<&u64> {
        self.ipsec_sas_offloaded.as_ref()
    }

    /// Sets the value of MulticastPacketsReceivedPersec
    pub fn set_multicast_packets_received_persec(&mut self, value: u64) {
        self.multicast_packets_received_persec = Some(value);
    }

    /// Gets the value of MulticastPacketsReceivedPersec
    pub fn get_multicast_packets_received_persec(&self) -> Option<&u64> {
        self.multicast_packets_received_persec.as_ref()
    }

    /// Sets the value of MulticastPacketsSentPersec
    pub fn set_multicast_packets_sent_persec(&mut self, value: u64) {
        self.multicast_packets_sent_persec = Some(value);
    }

    /// Gets the value of MulticastPacketsSentPersec
    pub fn get_multicast_packets_sent_persec(&self) -> Option<&u64> {
        self.multicast_packets_sent_persec.as_ref()
    }

    /// Sets the value of PacketsPersec
    pub fn set_packets_persec(&mut self, value: u64) {
        self.packets_persec = Some(value);
    }

    /// Gets the value of PacketsPersec
    pub fn get_packets_persec(&self) -> Option<&u64> {
        self.packets_persec.as_ref()
    }

    /// Sets the value of PacketsReceivedPersec
    pub fn set_packets_received_persec(&mut self, value: u64) {
        self.packets_received_persec = Some(value);
    }

    /// Gets the value of PacketsReceivedPersec
    pub fn get_packets_received_persec(&self) -> Option<&u64> {
        self.packets_received_persec.as_ref()
    }

    /// Sets the value of PacketsSentPersec
    pub fn set_packets_sent_persec(&mut self, value: u64) {
        self.packets_sent_persec = Some(value);
    }

    /// Gets the value of PacketsSentPersec
    pub fn get_packets_sent_persec(&self) -> Option<&u64> {
        self.packets_sent_persec.as_ref()
    }

    /// Sets the value of UnhashedPacketsReceivedPersec
    pub fn set_unhashed_packets_received_persec(&mut self, value: u64) {
        self.unhashed_packets_received_persec = Some(value);
    }

    /// Gets the value of UnhashedPacketsReceivedPersec
    pub fn get_unhashed_packets_received_persec(&self) -> Option<&u64> {
        self.unhashed_packets_received_persec.as_ref()
    }

    /// Sets the value of UnhashedPacketsSendCompletedPersec
    pub fn set_unhashed_packets_send_completed_persec(&mut self, value: u64) {
        self.unhashed_packets_send_completed_persec = Some(value);
    }

    /// Gets the value of UnhashedPacketsSendCompletedPersec
    pub fn get_unhashed_packets_send_completed_persec(&self) -> Option<&u64> {
        self.unhashed_packets_send_completed_persec.as_ref()
    }
}

