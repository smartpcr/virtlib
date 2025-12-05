// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_NvspSwitchStats_HyperVVirtualSwitch struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_NvspSwitchStats_HyperVVirtualSwitch {
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
    #[serde(rename = "LearnedMacAddresses")]
    pub learned_mac_addresses: Option<u64>,

/// 
    #[serde(rename = "LearnedMacAddressesPersec")]
    pub learned_mac_addresses_persec: Option<u64>,

/// 
    #[serde(rename = "MulticastPacketsReceivedPersec")]
    pub multicast_packets_received_persec: Option<u64>,

/// 
    #[serde(rename = "MulticastPacketsSentPersec")]
    pub multicast_packets_sent_persec: Option<u64>,

/// 
    #[serde(rename = "NumberofSendChannelMovesPersec")]
    pub numberof_send_channel_moves_persec: Option<u64>,

/// 
    #[serde(rename = "NumberofVMQMovesPersec")]
    pub numberof_vmqmoves_persec: Option<u64>,

/// 
    #[serde(rename = "PacketsFlooded")]
    pub packets_flooded: Option<u64>,

/// 
    #[serde(rename = "PacketsFloodedPersec")]
    pub packets_flooded_persec: Option<u64>,

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
    #[serde(rename = "PurgedMacAddresses")]
    pub purged_mac_addresses: Option<u64>,

/// 
    #[serde(rename = "PurgedMacAddressesPersec")]
    pub purged_mac_addresses_persec: Option<u64>,

/// 
    #[serde(rename = "RSCCoalescedBytes")]
    pub rsccoalesced_bytes: Option<u64>,

/// 
    #[serde(rename = "RSCCoalescedEventBucket10To1")]
    pub rsccoalesced_event_bucket10_to1: Option<u64>,

/// 
    #[serde(rename = "RSCCoalescedEventBucket22To3")]
    pub rsccoalesced_event_bucket22_to3: Option<u64>,

/// 
    #[serde(rename = "RSCCoalescedEventBucket34To7")]
    pub rsccoalesced_event_bucket34_to7: Option<u64>,

/// 
    #[serde(rename = "RSCCoalescedEventBucket48To15")]
    pub rsccoalesced_event_bucket48_to15: Option<u64>,

/// 
    #[serde(rename = "RSCCoalescedEventBucket516To31")]
    pub rsccoalesced_event_bucket516_to31: Option<u64>,

/// 
    #[serde(rename = "RSCCoalescedEventBucket632To63")]
    pub rsccoalesced_event_bucket632_to63: Option<u64>,

/// 
    #[serde(rename = "RSCCoalescedPacketBucket10To1")]
    pub rsccoalesced_packet_bucket10_to1: Option<u64>,

/// 
    #[serde(rename = "RSCCoalescedPacketBucket22To3")]
    pub rsccoalesced_packet_bucket22_to3: Option<u64>,

/// 
    #[serde(rename = "RSCCoalescedPacketBucket34To7")]
    pub rsccoalesced_packet_bucket34_to7: Option<u64>,

/// 
    #[serde(rename = "RSCCoalescedPacketBucket48To15")]
    pub rsccoalesced_packet_bucket48_to15: Option<u64>,

/// 
    #[serde(rename = "RSCCoalescedPacketBucket516To31")]
    pub rsccoalesced_packet_bucket516_to31: Option<u64>,

/// 
    #[serde(rename = "RSCCoalescedPacketBucket632To63")]
    pub rsccoalesced_packet_bucket632_to63: Option<u64>,

/// 
    #[serde(rename = "RSCCoalescedPackets")]
    pub rsccoalesced_packets: Option<u64>,

/// 
    #[serde(rename = "RSCCoalesceEvents")]
    pub rsccoalesce_events: Option<u64>,

/// 
    #[serde(rename = "RSCPacketsProcessed")]
    pub rscpackets_processed: Option<u64>,
}

impl Win32_PerfFormattedData_NvspSwitchStats_HyperVVirtualSwitch {
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
            learned_mac_addresses: None,
            learned_mac_addresses_persec: None,
            multicast_packets_received_persec: None,
            multicast_packets_sent_persec: None,
            numberof_send_channel_moves_persec: None,
            numberof_vmqmoves_persec: None,
            packets_flooded: None,
            packets_flooded_persec: None,
            packets_persec: None,
            packets_received_persec: None,
            packets_sent_persec: None,
            purged_mac_addresses: None,
            purged_mac_addresses_persec: None,
            rsccoalesced_bytes: None,
            rsccoalesced_event_bucket10_to1: None,
            rsccoalesced_event_bucket22_to3: None,
            rsccoalesced_event_bucket34_to7: None,
            rsccoalesced_event_bucket48_to15: None,
            rsccoalesced_event_bucket516_to31: None,
            rsccoalesced_event_bucket632_to63: None,
            rsccoalesced_packet_bucket10_to1: None,
            rsccoalesced_packet_bucket22_to3: None,
            rsccoalesced_packet_bucket34_to7: None,
            rsccoalesced_packet_bucket48_to15: None,
            rsccoalesced_packet_bucket516_to31: None,
            rsccoalesced_packet_bucket632_to63: None,
            rsccoalesced_packets: None,
            rsccoalesce_events: None,
            rscpackets_processed: None,
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

    /// Sets the value of LearnedMacAddresses
    pub fn set_learned_mac_addresses(&mut self, value: u64) {
        self.learned_mac_addresses = Some(value);
    }

    /// Gets the value of LearnedMacAddresses
    pub fn get_learned_mac_addresses(&self) -> Option<&u64> {
        self.learned_mac_addresses.as_ref()
    }

    /// Sets the value of LearnedMacAddressesPersec
    pub fn set_learned_mac_addresses_persec(&mut self, value: u64) {
        self.learned_mac_addresses_persec = Some(value);
    }

    /// Gets the value of LearnedMacAddressesPersec
    pub fn get_learned_mac_addresses_persec(&self) -> Option<&u64> {
        self.learned_mac_addresses_persec.as_ref()
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

    /// Sets the value of NumberofSendChannelMovesPersec
    pub fn set_numberof_send_channel_moves_persec(&mut self, value: u64) {
        self.numberof_send_channel_moves_persec = Some(value);
    }

    /// Gets the value of NumberofSendChannelMovesPersec
    pub fn get_numberof_send_channel_moves_persec(&self) -> Option<&u64> {
        self.numberof_send_channel_moves_persec.as_ref()
    }

    /// Sets the value of NumberofVMQMovesPersec
    pub fn set_numberof_vmqmoves_persec(&mut self, value: u64) {
        self.numberof_vmqmoves_persec = Some(value);
    }

    /// Gets the value of NumberofVMQMovesPersec
    pub fn get_numberof_vmqmoves_persec(&self) -> Option<&u64> {
        self.numberof_vmqmoves_persec.as_ref()
    }

    /// Sets the value of PacketsFlooded
    pub fn set_packets_flooded(&mut self, value: u64) {
        self.packets_flooded = Some(value);
    }

    /// Gets the value of PacketsFlooded
    pub fn get_packets_flooded(&self) -> Option<&u64> {
        self.packets_flooded.as_ref()
    }

    /// Sets the value of PacketsFloodedPersec
    pub fn set_packets_flooded_persec(&mut self, value: u64) {
        self.packets_flooded_persec = Some(value);
    }

    /// Gets the value of PacketsFloodedPersec
    pub fn get_packets_flooded_persec(&self) -> Option<&u64> {
        self.packets_flooded_persec.as_ref()
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

    /// Sets the value of PurgedMacAddresses
    pub fn set_purged_mac_addresses(&mut self, value: u64) {
        self.purged_mac_addresses = Some(value);
    }

    /// Gets the value of PurgedMacAddresses
    pub fn get_purged_mac_addresses(&self) -> Option<&u64> {
        self.purged_mac_addresses.as_ref()
    }

    /// Sets the value of PurgedMacAddressesPersec
    pub fn set_purged_mac_addresses_persec(&mut self, value: u64) {
        self.purged_mac_addresses_persec = Some(value);
    }

    /// Gets the value of PurgedMacAddressesPersec
    pub fn get_purged_mac_addresses_persec(&self) -> Option<&u64> {
        self.purged_mac_addresses_persec.as_ref()
    }

    /// Sets the value of RSCCoalescedBytes
    pub fn set_rsccoalesced_bytes(&mut self, value: u64) {
        self.rsccoalesced_bytes = Some(value);
    }

    /// Gets the value of RSCCoalescedBytes
    pub fn get_rsccoalesced_bytes(&self) -> Option<&u64> {
        self.rsccoalesced_bytes.as_ref()
    }

    /// Sets the value of RSCCoalescedEventBucket10To1
    pub fn set_rsccoalesced_event_bucket10_to1(&mut self, value: u64) {
        self.rsccoalesced_event_bucket10_to1 = Some(value);
    }

    /// Gets the value of RSCCoalescedEventBucket10To1
    pub fn get_rsccoalesced_event_bucket10_to1(&self) -> Option<&u64> {
        self.rsccoalesced_event_bucket10_to1.as_ref()
    }

    /// Sets the value of RSCCoalescedEventBucket22To3
    pub fn set_rsccoalesced_event_bucket22_to3(&mut self, value: u64) {
        self.rsccoalesced_event_bucket22_to3 = Some(value);
    }

    /// Gets the value of RSCCoalescedEventBucket22To3
    pub fn get_rsccoalesced_event_bucket22_to3(&self) -> Option<&u64> {
        self.rsccoalesced_event_bucket22_to3.as_ref()
    }

    /// Sets the value of RSCCoalescedEventBucket34To7
    pub fn set_rsccoalesced_event_bucket34_to7(&mut self, value: u64) {
        self.rsccoalesced_event_bucket34_to7 = Some(value);
    }

    /// Gets the value of RSCCoalescedEventBucket34To7
    pub fn get_rsccoalesced_event_bucket34_to7(&self) -> Option<&u64> {
        self.rsccoalesced_event_bucket34_to7.as_ref()
    }

    /// Sets the value of RSCCoalescedEventBucket48To15
    pub fn set_rsccoalesced_event_bucket48_to15(&mut self, value: u64) {
        self.rsccoalesced_event_bucket48_to15 = Some(value);
    }

    /// Gets the value of RSCCoalescedEventBucket48To15
    pub fn get_rsccoalesced_event_bucket48_to15(&self) -> Option<&u64> {
        self.rsccoalesced_event_bucket48_to15.as_ref()
    }

    /// Sets the value of RSCCoalescedEventBucket516To31
    pub fn set_rsccoalesced_event_bucket516_to31(&mut self, value: u64) {
        self.rsccoalesced_event_bucket516_to31 = Some(value);
    }

    /// Gets the value of RSCCoalescedEventBucket516To31
    pub fn get_rsccoalesced_event_bucket516_to31(&self) -> Option<&u64> {
        self.rsccoalesced_event_bucket516_to31.as_ref()
    }

    /// Sets the value of RSCCoalescedEventBucket632To63
    pub fn set_rsccoalesced_event_bucket632_to63(&mut self, value: u64) {
        self.rsccoalesced_event_bucket632_to63 = Some(value);
    }

    /// Gets the value of RSCCoalescedEventBucket632To63
    pub fn get_rsccoalesced_event_bucket632_to63(&self) -> Option<&u64> {
        self.rsccoalesced_event_bucket632_to63.as_ref()
    }

    /// Sets the value of RSCCoalescedPacketBucket10To1
    pub fn set_rsccoalesced_packet_bucket10_to1(&mut self, value: u64) {
        self.rsccoalesced_packet_bucket10_to1 = Some(value);
    }

    /// Gets the value of RSCCoalescedPacketBucket10To1
    pub fn get_rsccoalesced_packet_bucket10_to1(&self) -> Option<&u64> {
        self.rsccoalesced_packet_bucket10_to1.as_ref()
    }

    /// Sets the value of RSCCoalescedPacketBucket22To3
    pub fn set_rsccoalesced_packet_bucket22_to3(&mut self, value: u64) {
        self.rsccoalesced_packet_bucket22_to3 = Some(value);
    }

    /// Gets the value of RSCCoalescedPacketBucket22To3
    pub fn get_rsccoalesced_packet_bucket22_to3(&self) -> Option<&u64> {
        self.rsccoalesced_packet_bucket22_to3.as_ref()
    }

    /// Sets the value of RSCCoalescedPacketBucket34To7
    pub fn set_rsccoalesced_packet_bucket34_to7(&mut self, value: u64) {
        self.rsccoalesced_packet_bucket34_to7 = Some(value);
    }

    /// Gets the value of RSCCoalescedPacketBucket34To7
    pub fn get_rsccoalesced_packet_bucket34_to7(&self) -> Option<&u64> {
        self.rsccoalesced_packet_bucket34_to7.as_ref()
    }

    /// Sets the value of RSCCoalescedPacketBucket48To15
    pub fn set_rsccoalesced_packet_bucket48_to15(&mut self, value: u64) {
        self.rsccoalesced_packet_bucket48_to15 = Some(value);
    }

    /// Gets the value of RSCCoalescedPacketBucket48To15
    pub fn get_rsccoalesced_packet_bucket48_to15(&self) -> Option<&u64> {
        self.rsccoalesced_packet_bucket48_to15.as_ref()
    }

    /// Sets the value of RSCCoalescedPacketBucket516To31
    pub fn set_rsccoalesced_packet_bucket516_to31(&mut self, value: u64) {
        self.rsccoalesced_packet_bucket516_to31 = Some(value);
    }

    /// Gets the value of RSCCoalescedPacketBucket516To31
    pub fn get_rsccoalesced_packet_bucket516_to31(&self) -> Option<&u64> {
        self.rsccoalesced_packet_bucket516_to31.as_ref()
    }

    /// Sets the value of RSCCoalescedPacketBucket632To63
    pub fn set_rsccoalesced_packet_bucket632_to63(&mut self, value: u64) {
        self.rsccoalesced_packet_bucket632_to63 = Some(value);
    }

    /// Gets the value of RSCCoalescedPacketBucket632To63
    pub fn get_rsccoalesced_packet_bucket632_to63(&self) -> Option<&u64> {
        self.rsccoalesced_packet_bucket632_to63.as_ref()
    }

    /// Sets the value of RSCCoalescedPackets
    pub fn set_rsccoalesced_packets(&mut self, value: u64) {
        self.rsccoalesced_packets = Some(value);
    }

    /// Gets the value of RSCCoalescedPackets
    pub fn get_rsccoalesced_packets(&self) -> Option<&u64> {
        self.rsccoalesced_packets.as_ref()
    }

    /// Sets the value of RSCCoalesceEvents
    pub fn set_rsccoalesce_events(&mut self, value: u64) {
        self.rsccoalesce_events = Some(value);
    }

    /// Gets the value of RSCCoalesceEvents
    pub fn get_rsccoalesce_events(&self) -> Option<&u64> {
        self.rsccoalesce_events.as_ref()
    }

    /// Sets the value of RSCPacketsProcessed
    pub fn set_rscpackets_processed(&mut self, value: u64) {
        self.rscpackets_processed = Some(value);
    }

    /// Gets the value of RSCPacketsProcessed
    pub fn get_rscpackets_processed(&self) -> Option<&u64> {
        self.rscpackets_processed.as_ref()
    }
}

