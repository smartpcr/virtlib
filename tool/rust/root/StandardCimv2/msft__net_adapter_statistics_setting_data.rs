// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterStatisticsSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterStatisticsSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterSettingData,

/// 
    #[serde(rename = "OutboundDiscardedPackets")]
    pub outbound_discarded_packets: Option<u64>,

/// 
    #[serde(rename = "OutboundPacketErrors")]
    pub outbound_packet_errors: Option<u64>,

/// 
    #[serde(rename = "RdmaStatistics")]
    pub rdma_statistics: Option<MSFT_NetAdapter_RdmaStatistics>,

/// 
    #[serde(rename = "ReceivedBroadcastBytes")]
    pub received_broadcast_bytes: Option<u64>,

/// 
    #[serde(rename = "ReceivedBroadcastPackets")]
    pub received_broadcast_packets: Option<u64>,

/// 
    #[serde(rename = "ReceivedBytes")]
    pub received_bytes: Option<u64>,

/// 
    #[serde(rename = "ReceivedDiscardedPackets")]
    pub received_discarded_packets: Option<u64>,

/// 
    #[serde(rename = "ReceivedMulticastBytes")]
    pub received_multicast_bytes: Option<u64>,

/// 
    #[serde(rename = "ReceivedMulticastPackets")]
    pub received_multicast_packets: Option<u64>,

/// 
    #[serde(rename = "ReceivedPacketErrors")]
    pub received_packet_errors: Option<u64>,

/// 
    #[serde(rename = "ReceivedUnicastBytes")]
    pub received_unicast_bytes: Option<u64>,

/// 
    #[serde(rename = "ReceivedUnicastPackets")]
    pub received_unicast_packets: Option<u64>,

/// 
    #[serde(rename = "RscStatistics")]
    pub rsc_statistics: Option<MSFT_NetAdapter_RscStatistics>,

/// 
    #[serde(rename = "SentBroadcastBytes")]
    pub sent_broadcast_bytes: Option<u64>,

/// 
    #[serde(rename = "SentBroadcastPackets")]
    pub sent_broadcast_packets: Option<u64>,

/// 
    #[serde(rename = "SentBytes")]
    pub sent_bytes: Option<u64>,

/// 
    #[serde(rename = "SentMulticastBytes")]
    pub sent_multicast_bytes: Option<u64>,

/// 
    #[serde(rename = "SentMulticastPackets")]
    pub sent_multicast_packets: Option<u64>,

/// 
    #[serde(rename = "SentUnicastBytes")]
    pub sent_unicast_bytes: Option<u64>,

/// 
    #[serde(rename = "SentUnicastPackets")]
    pub sent_unicast_packets: Option<u64>,

/// 
    #[serde(rename = "SupportedStatistics")]
    pub supported_statistics: Option<u32>,
}

impl MSFT_NetAdapterStatisticsSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterSettingData::new(),
            outbound_discarded_packets: None,
            outbound_packet_errors: None,
            rdma_statistics: None,
            received_broadcast_bytes: None,
            received_broadcast_packets: None,
            received_bytes: None,
            received_discarded_packets: None,
            received_multicast_bytes: None,
            received_multicast_packets: None,
            received_packet_errors: None,
            received_unicast_bytes: None,
            received_unicast_packets: None,
            rsc_statistics: None,
            sent_broadcast_bytes: None,
            sent_broadcast_packets: None,
            sent_bytes: None,
            sent_multicast_bytes: None,
            sent_multicast_packets: None,
            sent_unicast_bytes: None,
            sent_unicast_packets: None,
            supported_statistics: None,
        }
    }


    /// Sets the value of OutboundDiscardedPackets
    pub fn set_outbound_discarded_packets(&mut self, value: u64) {
        self.outbound_discarded_packets = Some(value);
    }

    /// Gets the value of OutboundDiscardedPackets
    pub fn get_outbound_discarded_packets(&self) -> Option<&u64> {
        self.outbound_discarded_packets.as_ref()
    }

    /// Sets the value of OutboundPacketErrors
    pub fn set_outbound_packet_errors(&mut self, value: u64) {
        self.outbound_packet_errors = Some(value);
    }

    /// Gets the value of OutboundPacketErrors
    pub fn get_outbound_packet_errors(&self) -> Option<&u64> {
        self.outbound_packet_errors.as_ref()
    }

    /// Sets the value of RdmaStatistics
    pub fn set_rdma_statistics(&mut self, value: MSFT_NetAdapter_RdmaStatistics) {
        self.rdma_statistics = Some(value);
    }

    /// Gets the value of RdmaStatistics
    pub fn get_rdma_statistics(&self) -> Option<&MSFT_NetAdapter_RdmaStatistics> {
        self.rdma_statistics.as_ref()
    }

    /// Sets the value of ReceivedBroadcastBytes
    pub fn set_received_broadcast_bytes(&mut self, value: u64) {
        self.received_broadcast_bytes = Some(value);
    }

    /// Gets the value of ReceivedBroadcastBytes
    pub fn get_received_broadcast_bytes(&self) -> Option<&u64> {
        self.received_broadcast_bytes.as_ref()
    }

    /// Sets the value of ReceivedBroadcastPackets
    pub fn set_received_broadcast_packets(&mut self, value: u64) {
        self.received_broadcast_packets = Some(value);
    }

    /// Gets the value of ReceivedBroadcastPackets
    pub fn get_received_broadcast_packets(&self) -> Option<&u64> {
        self.received_broadcast_packets.as_ref()
    }

    /// Sets the value of ReceivedBytes
    pub fn set_received_bytes(&mut self, value: u64) {
        self.received_bytes = Some(value);
    }

    /// Gets the value of ReceivedBytes
    pub fn get_received_bytes(&self) -> Option<&u64> {
        self.received_bytes.as_ref()
    }

    /// Sets the value of ReceivedDiscardedPackets
    pub fn set_received_discarded_packets(&mut self, value: u64) {
        self.received_discarded_packets = Some(value);
    }

    /// Gets the value of ReceivedDiscardedPackets
    pub fn get_received_discarded_packets(&self) -> Option<&u64> {
        self.received_discarded_packets.as_ref()
    }

    /// Sets the value of ReceivedMulticastBytes
    pub fn set_received_multicast_bytes(&mut self, value: u64) {
        self.received_multicast_bytes = Some(value);
    }

    /// Gets the value of ReceivedMulticastBytes
    pub fn get_received_multicast_bytes(&self) -> Option<&u64> {
        self.received_multicast_bytes.as_ref()
    }

    /// Sets the value of ReceivedMulticastPackets
    pub fn set_received_multicast_packets(&mut self, value: u64) {
        self.received_multicast_packets = Some(value);
    }

    /// Gets the value of ReceivedMulticastPackets
    pub fn get_received_multicast_packets(&self) -> Option<&u64> {
        self.received_multicast_packets.as_ref()
    }

    /// Sets the value of ReceivedPacketErrors
    pub fn set_received_packet_errors(&mut self, value: u64) {
        self.received_packet_errors = Some(value);
    }

    /// Gets the value of ReceivedPacketErrors
    pub fn get_received_packet_errors(&self) -> Option<&u64> {
        self.received_packet_errors.as_ref()
    }

    /// Sets the value of ReceivedUnicastBytes
    pub fn set_received_unicast_bytes(&mut self, value: u64) {
        self.received_unicast_bytes = Some(value);
    }

    /// Gets the value of ReceivedUnicastBytes
    pub fn get_received_unicast_bytes(&self) -> Option<&u64> {
        self.received_unicast_bytes.as_ref()
    }

    /// Sets the value of ReceivedUnicastPackets
    pub fn set_received_unicast_packets(&mut self, value: u64) {
        self.received_unicast_packets = Some(value);
    }

    /// Gets the value of ReceivedUnicastPackets
    pub fn get_received_unicast_packets(&self) -> Option<&u64> {
        self.received_unicast_packets.as_ref()
    }

    /// Sets the value of RscStatistics
    pub fn set_rsc_statistics(&mut self, value: MSFT_NetAdapter_RscStatistics) {
        self.rsc_statistics = Some(value);
    }

    /// Gets the value of RscStatistics
    pub fn get_rsc_statistics(&self) -> Option<&MSFT_NetAdapter_RscStatistics> {
        self.rsc_statistics.as_ref()
    }

    /// Sets the value of SentBroadcastBytes
    pub fn set_sent_broadcast_bytes(&mut self, value: u64) {
        self.sent_broadcast_bytes = Some(value);
    }

    /// Gets the value of SentBroadcastBytes
    pub fn get_sent_broadcast_bytes(&self) -> Option<&u64> {
        self.sent_broadcast_bytes.as_ref()
    }

    /// Sets the value of SentBroadcastPackets
    pub fn set_sent_broadcast_packets(&mut self, value: u64) {
        self.sent_broadcast_packets = Some(value);
    }

    /// Gets the value of SentBroadcastPackets
    pub fn get_sent_broadcast_packets(&self) -> Option<&u64> {
        self.sent_broadcast_packets.as_ref()
    }

    /// Sets the value of SentBytes
    pub fn set_sent_bytes(&mut self, value: u64) {
        self.sent_bytes = Some(value);
    }

    /// Gets the value of SentBytes
    pub fn get_sent_bytes(&self) -> Option<&u64> {
        self.sent_bytes.as_ref()
    }

    /// Sets the value of SentMulticastBytes
    pub fn set_sent_multicast_bytes(&mut self, value: u64) {
        self.sent_multicast_bytes = Some(value);
    }

    /// Gets the value of SentMulticastBytes
    pub fn get_sent_multicast_bytes(&self) -> Option<&u64> {
        self.sent_multicast_bytes.as_ref()
    }

    /// Sets the value of SentMulticastPackets
    pub fn set_sent_multicast_packets(&mut self, value: u64) {
        self.sent_multicast_packets = Some(value);
    }

    /// Gets the value of SentMulticastPackets
    pub fn get_sent_multicast_packets(&self) -> Option<&u64> {
        self.sent_multicast_packets.as_ref()
    }

    /// Sets the value of SentUnicastBytes
    pub fn set_sent_unicast_bytes(&mut self, value: u64) {
        self.sent_unicast_bytes = Some(value);
    }

    /// Gets the value of SentUnicastBytes
    pub fn get_sent_unicast_bytes(&self) -> Option<&u64> {
        self.sent_unicast_bytes.as_ref()
    }

    /// Sets the value of SentUnicastPackets
    pub fn set_sent_unicast_packets(&mut self, value: u64) {
        self.sent_unicast_packets = Some(value);
    }

    /// Gets the value of SentUnicastPackets
    pub fn get_sent_unicast_packets(&self) -> Option<&u64> {
        self.sent_unicast_packets.as_ref()
    }

    /// Sets the value of SupportedStatistics
    pub fn set_supported_statistics(&mut self, value: u32) {
        self.supported_statistics = Some(value);
    }

    /// Gets the value of SupportedStatistics
    pub fn get_supported_statistics(&self) -> Option<&u32> {
        self.supported_statistics.as_ref()
    }
}

