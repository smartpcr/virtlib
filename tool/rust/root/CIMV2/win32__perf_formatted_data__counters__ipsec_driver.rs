// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_IPsecDriver struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_IPsecDriver {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "ActiveSecurityAssociations")]
    pub active_security_associations: Option<u32>,

/// 
    #[serde(rename = "BytesReceivedinTransportModePersec")]
    pub bytes_receivedin_transport_mode_persec: Option<u32>,

/// 
    #[serde(rename = "BytesReceivedinTunnelModePersec")]
    pub bytes_receivedin_tunnel_mode_persec: Option<u32>,

/// 
    #[serde(rename = "BytesSentinTransportModePersec")]
    pub bytes_sentin_transport_mode_persec: Option<u32>,

/// 
    #[serde(rename = "BytesSentinTunnelModePersec")]
    pub bytes_sentin_tunnel_mode_persec: Option<u32>,

/// 
    #[serde(rename = "InboundPacketsDroppedPersec")]
    pub inbound_packets_dropped_persec: Option<u32>,

/// 
    #[serde(rename = "InboundPacketsReceivedPersec")]
    pub inbound_packets_received_persec: Option<u32>,

/// 
    #[serde(rename = "IncorrectSPIPackets")]
    pub incorrect_spipackets: Option<u32>,

/// 
    #[serde(rename = "IncorrectSPIPacketsPersec")]
    pub incorrect_spipackets_persec: Option<u32>,

/// 
    #[serde(rename = "OffloadedBytesReceivedPersec")]
    pub offloaded_bytes_received_persec: Option<u32>,

/// 
    #[serde(rename = "OffloadedBytesSentPersec")]
    pub offloaded_bytes_sent_persec: Option<u32>,

/// 
    #[serde(rename = "OffloadedSecurityAssociations")]
    pub offloaded_security_associations: Option<u32>,

/// 
    #[serde(rename = "PacketsNotAuthenticated")]
    pub packets_not_authenticated: Option<u32>,

/// 
    #[serde(rename = "PacketsNotAuthenticatedPersec")]
    pub packets_not_authenticated_persec: Option<u32>,

/// 
    #[serde(rename = "PacketsNotDecrypted")]
    pub packets_not_decrypted: Option<u32>,

/// 
    #[serde(rename = "PacketsNotDecryptedPersec")]
    pub packets_not_decrypted_persec: Option<u32>,

/// 
    #[serde(rename = "PacketsReceivedOverWrongSA")]
    pub packets_received_over_wrong_sa: Option<u32>,

/// 
    #[serde(rename = "PacketsReceivedOverWrongSAPersec")]
    pub packets_received_over_wrong_sapersec: Option<u32>,

/// 
    #[serde(rename = "PacketsThatFailedESPValidation")]
    pub packets_that_failed_espvalidation: Option<u32>,

/// 
    #[serde(rename = "PacketsThatFailedESPValidationPersec")]
    pub packets_that_failed_espvalidation_persec: Option<u32>,

/// 
    #[serde(rename = "PacketsThatFailedReplayDetection")]
    pub packets_that_failed_replay_detection: Option<u32>,

/// 
    #[serde(rename = "PacketsThatFailedReplayDetectionPersec")]
    pub packets_that_failed_replay_detection_persec: Option<u32>,

/// 
    #[serde(rename = "PacketsThatFailedUDPESPValidation")]
    pub packets_that_failed_udpespvalidation: Option<u32>,

/// 
    #[serde(rename = "PacketsThatFailedUDPESPValidationPersec")]
    pub packets_that_failed_udpespvalidation_persec: Option<u32>,

/// 
    #[serde(rename = "PendingSecurityAssociations")]
    pub pending_security_associations: Option<u32>,

/// 
    #[serde(rename = "PlaintextPacketsReceived")]
    pub plaintext_packets_received: Option<u32>,

/// 
    #[serde(rename = "PlaintextPacketsReceivedPersec")]
    pub plaintext_packets_received_persec: Option<u32>,

/// 
    #[serde(rename = "SARekeys")]
    pub sarekeys: Option<u32>,

/// 
    #[serde(rename = "SecurityAssociationsAdded")]
    pub security_associations_added: Option<u32>,

/// 
    #[serde(rename = "TotalInboundPacketsDropped")]
    pub total_inbound_packets_dropped: Option<u32>,

/// 
    #[serde(rename = "TotalInboundPacketsReceived")]
    pub total_inbound_packets_received: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_IPsecDriver {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            active_security_associations: None,
            bytes_receivedin_transport_mode_persec: None,
            bytes_receivedin_tunnel_mode_persec: None,
            bytes_sentin_transport_mode_persec: None,
            bytes_sentin_tunnel_mode_persec: None,
            inbound_packets_dropped_persec: None,
            inbound_packets_received_persec: None,
            incorrect_spipackets: None,
            incorrect_spipackets_persec: None,
            offloaded_bytes_received_persec: None,
            offloaded_bytes_sent_persec: None,
            offloaded_security_associations: None,
            packets_not_authenticated: None,
            packets_not_authenticated_persec: None,
            packets_not_decrypted: None,
            packets_not_decrypted_persec: None,
            packets_received_over_wrong_sa: None,
            packets_received_over_wrong_sapersec: None,
            packets_that_failed_espvalidation: None,
            packets_that_failed_espvalidation_persec: None,
            packets_that_failed_replay_detection: None,
            packets_that_failed_replay_detection_persec: None,
            packets_that_failed_udpespvalidation: None,
            packets_that_failed_udpespvalidation_persec: None,
            pending_security_associations: None,
            plaintext_packets_received: None,
            plaintext_packets_received_persec: None,
            sarekeys: None,
            security_associations_added: None,
            total_inbound_packets_dropped: None,
            total_inbound_packets_received: None,
        }
    }


    /// Sets the value of ActiveSecurityAssociations
    pub fn set_active_security_associations(&mut self, value: u32) {
        self.active_security_associations = Some(value);
    }

    /// Gets the value of ActiveSecurityAssociations
    pub fn get_active_security_associations(&self) -> Option<&u32> {
        self.active_security_associations.as_ref()
    }

    /// Sets the value of BytesReceivedinTransportModePersec
    pub fn set_bytes_receivedin_transport_mode_persec(&mut self, value: u32) {
        self.bytes_receivedin_transport_mode_persec = Some(value);
    }

    /// Gets the value of BytesReceivedinTransportModePersec
    pub fn get_bytes_receivedin_transport_mode_persec(&self) -> Option<&u32> {
        self.bytes_receivedin_transport_mode_persec.as_ref()
    }

    /// Sets the value of BytesReceivedinTunnelModePersec
    pub fn set_bytes_receivedin_tunnel_mode_persec(&mut self, value: u32) {
        self.bytes_receivedin_tunnel_mode_persec = Some(value);
    }

    /// Gets the value of BytesReceivedinTunnelModePersec
    pub fn get_bytes_receivedin_tunnel_mode_persec(&self) -> Option<&u32> {
        self.bytes_receivedin_tunnel_mode_persec.as_ref()
    }

    /// Sets the value of BytesSentinTransportModePersec
    pub fn set_bytes_sentin_transport_mode_persec(&mut self, value: u32) {
        self.bytes_sentin_transport_mode_persec = Some(value);
    }

    /// Gets the value of BytesSentinTransportModePersec
    pub fn get_bytes_sentin_transport_mode_persec(&self) -> Option<&u32> {
        self.bytes_sentin_transport_mode_persec.as_ref()
    }

    /// Sets the value of BytesSentinTunnelModePersec
    pub fn set_bytes_sentin_tunnel_mode_persec(&mut self, value: u32) {
        self.bytes_sentin_tunnel_mode_persec = Some(value);
    }

    /// Gets the value of BytesSentinTunnelModePersec
    pub fn get_bytes_sentin_tunnel_mode_persec(&self) -> Option<&u32> {
        self.bytes_sentin_tunnel_mode_persec.as_ref()
    }

    /// Sets the value of InboundPacketsDroppedPersec
    pub fn set_inbound_packets_dropped_persec(&mut self, value: u32) {
        self.inbound_packets_dropped_persec = Some(value);
    }

    /// Gets the value of InboundPacketsDroppedPersec
    pub fn get_inbound_packets_dropped_persec(&self) -> Option<&u32> {
        self.inbound_packets_dropped_persec.as_ref()
    }

    /// Sets the value of InboundPacketsReceivedPersec
    pub fn set_inbound_packets_received_persec(&mut self, value: u32) {
        self.inbound_packets_received_persec = Some(value);
    }

    /// Gets the value of InboundPacketsReceivedPersec
    pub fn get_inbound_packets_received_persec(&self) -> Option<&u32> {
        self.inbound_packets_received_persec.as_ref()
    }

    /// Sets the value of IncorrectSPIPackets
    pub fn set_incorrect_spipackets(&mut self, value: u32) {
        self.incorrect_spipackets = Some(value);
    }

    /// Gets the value of IncorrectSPIPackets
    pub fn get_incorrect_spipackets(&self) -> Option<&u32> {
        self.incorrect_spipackets.as_ref()
    }

    /// Sets the value of IncorrectSPIPacketsPersec
    pub fn set_incorrect_spipackets_persec(&mut self, value: u32) {
        self.incorrect_spipackets_persec = Some(value);
    }

    /// Gets the value of IncorrectSPIPacketsPersec
    pub fn get_incorrect_spipackets_persec(&self) -> Option<&u32> {
        self.incorrect_spipackets_persec.as_ref()
    }

    /// Sets the value of OffloadedBytesReceivedPersec
    pub fn set_offloaded_bytes_received_persec(&mut self, value: u32) {
        self.offloaded_bytes_received_persec = Some(value);
    }

    /// Gets the value of OffloadedBytesReceivedPersec
    pub fn get_offloaded_bytes_received_persec(&self) -> Option<&u32> {
        self.offloaded_bytes_received_persec.as_ref()
    }

    /// Sets the value of OffloadedBytesSentPersec
    pub fn set_offloaded_bytes_sent_persec(&mut self, value: u32) {
        self.offloaded_bytes_sent_persec = Some(value);
    }

    /// Gets the value of OffloadedBytesSentPersec
    pub fn get_offloaded_bytes_sent_persec(&self) -> Option<&u32> {
        self.offloaded_bytes_sent_persec.as_ref()
    }

    /// Sets the value of OffloadedSecurityAssociations
    pub fn set_offloaded_security_associations(&mut self, value: u32) {
        self.offloaded_security_associations = Some(value);
    }

    /// Gets the value of OffloadedSecurityAssociations
    pub fn get_offloaded_security_associations(&self) -> Option<&u32> {
        self.offloaded_security_associations.as_ref()
    }

    /// Sets the value of PacketsNotAuthenticated
    pub fn set_packets_not_authenticated(&mut self, value: u32) {
        self.packets_not_authenticated = Some(value);
    }

    /// Gets the value of PacketsNotAuthenticated
    pub fn get_packets_not_authenticated(&self) -> Option<&u32> {
        self.packets_not_authenticated.as_ref()
    }

    /// Sets the value of PacketsNotAuthenticatedPersec
    pub fn set_packets_not_authenticated_persec(&mut self, value: u32) {
        self.packets_not_authenticated_persec = Some(value);
    }

    /// Gets the value of PacketsNotAuthenticatedPersec
    pub fn get_packets_not_authenticated_persec(&self) -> Option<&u32> {
        self.packets_not_authenticated_persec.as_ref()
    }

    /// Sets the value of PacketsNotDecrypted
    pub fn set_packets_not_decrypted(&mut self, value: u32) {
        self.packets_not_decrypted = Some(value);
    }

    /// Gets the value of PacketsNotDecrypted
    pub fn get_packets_not_decrypted(&self) -> Option<&u32> {
        self.packets_not_decrypted.as_ref()
    }

    /// Sets the value of PacketsNotDecryptedPersec
    pub fn set_packets_not_decrypted_persec(&mut self, value: u32) {
        self.packets_not_decrypted_persec = Some(value);
    }

    /// Gets the value of PacketsNotDecryptedPersec
    pub fn get_packets_not_decrypted_persec(&self) -> Option<&u32> {
        self.packets_not_decrypted_persec.as_ref()
    }

    /// Sets the value of PacketsReceivedOverWrongSA
    pub fn set_packets_received_over_wrong_sa(&mut self, value: u32) {
        self.packets_received_over_wrong_sa = Some(value);
    }

    /// Gets the value of PacketsReceivedOverWrongSA
    pub fn get_packets_received_over_wrong_sa(&self) -> Option<&u32> {
        self.packets_received_over_wrong_sa.as_ref()
    }

    /// Sets the value of PacketsReceivedOverWrongSAPersec
    pub fn set_packets_received_over_wrong_sapersec(&mut self, value: u32) {
        self.packets_received_over_wrong_sapersec = Some(value);
    }

    /// Gets the value of PacketsReceivedOverWrongSAPersec
    pub fn get_packets_received_over_wrong_sapersec(&self) -> Option<&u32> {
        self.packets_received_over_wrong_sapersec.as_ref()
    }

    /// Sets the value of PacketsThatFailedESPValidation
    pub fn set_packets_that_failed_espvalidation(&mut self, value: u32) {
        self.packets_that_failed_espvalidation = Some(value);
    }

    /// Gets the value of PacketsThatFailedESPValidation
    pub fn get_packets_that_failed_espvalidation(&self) -> Option<&u32> {
        self.packets_that_failed_espvalidation.as_ref()
    }

    /// Sets the value of PacketsThatFailedESPValidationPersec
    pub fn set_packets_that_failed_espvalidation_persec(&mut self, value: u32) {
        self.packets_that_failed_espvalidation_persec = Some(value);
    }

    /// Gets the value of PacketsThatFailedESPValidationPersec
    pub fn get_packets_that_failed_espvalidation_persec(&self) -> Option<&u32> {
        self.packets_that_failed_espvalidation_persec.as_ref()
    }

    /// Sets the value of PacketsThatFailedReplayDetection
    pub fn set_packets_that_failed_replay_detection(&mut self, value: u32) {
        self.packets_that_failed_replay_detection = Some(value);
    }

    /// Gets the value of PacketsThatFailedReplayDetection
    pub fn get_packets_that_failed_replay_detection(&self) -> Option<&u32> {
        self.packets_that_failed_replay_detection.as_ref()
    }

    /// Sets the value of PacketsThatFailedReplayDetectionPersec
    pub fn set_packets_that_failed_replay_detection_persec(&mut self, value: u32) {
        self.packets_that_failed_replay_detection_persec = Some(value);
    }

    /// Gets the value of PacketsThatFailedReplayDetectionPersec
    pub fn get_packets_that_failed_replay_detection_persec(&self) -> Option<&u32> {
        self.packets_that_failed_replay_detection_persec.as_ref()
    }

    /// Sets the value of PacketsThatFailedUDPESPValidation
    pub fn set_packets_that_failed_udpespvalidation(&mut self, value: u32) {
        self.packets_that_failed_udpespvalidation = Some(value);
    }

    /// Gets the value of PacketsThatFailedUDPESPValidation
    pub fn get_packets_that_failed_udpespvalidation(&self) -> Option<&u32> {
        self.packets_that_failed_udpespvalidation.as_ref()
    }

    /// Sets the value of PacketsThatFailedUDPESPValidationPersec
    pub fn set_packets_that_failed_udpespvalidation_persec(&mut self, value: u32) {
        self.packets_that_failed_udpespvalidation_persec = Some(value);
    }

    /// Gets the value of PacketsThatFailedUDPESPValidationPersec
    pub fn get_packets_that_failed_udpespvalidation_persec(&self) -> Option<&u32> {
        self.packets_that_failed_udpespvalidation_persec.as_ref()
    }

    /// Sets the value of PendingSecurityAssociations
    pub fn set_pending_security_associations(&mut self, value: u32) {
        self.pending_security_associations = Some(value);
    }

    /// Gets the value of PendingSecurityAssociations
    pub fn get_pending_security_associations(&self) -> Option<&u32> {
        self.pending_security_associations.as_ref()
    }

    /// Sets the value of PlaintextPacketsReceived
    pub fn set_plaintext_packets_received(&mut self, value: u32) {
        self.plaintext_packets_received = Some(value);
    }

    /// Gets the value of PlaintextPacketsReceived
    pub fn get_plaintext_packets_received(&self) -> Option<&u32> {
        self.plaintext_packets_received.as_ref()
    }

    /// Sets the value of PlaintextPacketsReceivedPersec
    pub fn set_plaintext_packets_received_persec(&mut self, value: u32) {
        self.plaintext_packets_received_persec = Some(value);
    }

    /// Gets the value of PlaintextPacketsReceivedPersec
    pub fn get_plaintext_packets_received_persec(&self) -> Option<&u32> {
        self.plaintext_packets_received_persec.as_ref()
    }

    /// Sets the value of SARekeys
    pub fn set_sarekeys(&mut self, value: u32) {
        self.sarekeys = Some(value);
    }

    /// Gets the value of SARekeys
    pub fn get_sarekeys(&self) -> Option<&u32> {
        self.sarekeys.as_ref()
    }

    /// Sets the value of SecurityAssociationsAdded
    pub fn set_security_associations_added(&mut self, value: u32) {
        self.security_associations_added = Some(value);
    }

    /// Gets the value of SecurityAssociationsAdded
    pub fn get_security_associations_added(&self) -> Option<&u32> {
        self.security_associations_added.as_ref()
    }

    /// Sets the value of TotalInboundPacketsDropped
    pub fn set_total_inbound_packets_dropped(&mut self, value: u32) {
        self.total_inbound_packets_dropped = Some(value);
    }

    /// Gets the value of TotalInboundPacketsDropped
    pub fn get_total_inbound_packets_dropped(&self) -> Option<&u32> {
        self.total_inbound_packets_dropped.as_ref()
    }

    /// Sets the value of TotalInboundPacketsReceived
    pub fn set_total_inbound_packets_received(&mut self, value: u32) {
        self.total_inbound_packets_received = Some(value);
    }

    /// Gets the value of TotalInboundPacketsReceived
    pub fn get_total_inbound_packets_received(&self) -> Option<&u32> {
        self.total_inbound_packets_received.as_ref()
    }
}

