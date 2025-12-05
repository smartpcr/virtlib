// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_WFPReauthorization struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_WFPReauthorization {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "FamilyV4")]
    pub family_v4: Option<u64>,

/// 
    #[serde(rename = "FamilyV6")]
    pub family_v6: Option<u64>,

/// 
    #[serde(rename = "Inbound")]
    pub inbound: Option<u64>,

/// 
    #[serde(rename = "Outbound")]
    pub outbound: Option<u64>,

/// 
    #[serde(rename = "ProtocolICMP")]
    pub protocol_icmp: Option<u64>,

/// 
    #[serde(rename = "ProtocolICMP6")]
    pub protocol_icmp6: Option<u64>,

/// 
    #[serde(rename = "ProtocolIPv4")]
    pub protocol_ipv4: Option<u64>,

/// 
    #[serde(rename = "ProtocolIPv6")]
    pub protocol_ipv6: Option<u64>,

/// 
    #[serde(rename = "ProtocolOther")]
    pub protocol_other: Option<u64>,

/// 
    #[serde(rename = "ProtocolTCP")]
    pub protocol_tcp: Option<u64>,

/// 
    #[serde(rename = "ProtocolUDP")]
    pub protocol_udp: Option<u64>,

/// 
    #[serde(rename = "ReasonClassifyCompletion")]
    pub reason_classify_completion: Option<u64>,

/// 
    #[serde(rename = "ReasonEDPPolicyChanged")]
    pub reason_edppolicy_changed: Option<u64>,

/// 
    #[serde(rename = "ReasonIPSecPropertiesChanged")]
    pub reason_ipsec_properties_changed: Option<u64>,

/// 
    #[serde(rename = "ReasonMidStreamInspection")]
    pub reason_mid_stream_inspection: Option<u64>,

/// 
    #[serde(rename = "ReasonNewArrivalInterface")]
    pub reason_new_arrival_interface: Option<u64>,

/// 
    #[serde(rename = "ReasonNewInboundMCastBCastPacket")]
    pub reason_new_inbound_mcast_bcast_packet: Option<u64>,

/// 
    #[serde(rename = "ReasonNewNextHopInterface")]
    pub reason_new_next_hop_interface: Option<u64>,

/// 
    #[serde(rename = "ReasonPolicyChange")]
    pub reason_policy_change: Option<u64>,

/// 
    #[serde(rename = "ReasonProfileCrossing")]
    pub reason_profile_crossing: Option<u64>,

/// 
    #[serde(rename = "ReasonProxyHandleChanged")]
    pub reason_proxy_handle_changed: Option<u64>,

/// 
    #[serde(rename = "ReasonSocketPropertyChanged")]
    pub reason_socket_property_changed: Option<u64>,
}

impl Win32_PerfFormattedData_Counters_WFPReauthorization {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            family_v4: None,
            family_v6: None,
            inbound: None,
            outbound: None,
            protocol_icmp: None,
            protocol_icmp6: None,
            protocol_ipv4: None,
            protocol_ipv6: None,
            protocol_other: None,
            protocol_tcp: None,
            protocol_udp: None,
            reason_classify_completion: None,
            reason_edppolicy_changed: None,
            reason_ipsec_properties_changed: None,
            reason_mid_stream_inspection: None,
            reason_new_arrival_interface: None,
            reason_new_inbound_mcast_bcast_packet: None,
            reason_new_next_hop_interface: None,
            reason_policy_change: None,
            reason_profile_crossing: None,
            reason_proxy_handle_changed: None,
            reason_socket_property_changed: None,
        }
    }


    /// Sets the value of FamilyV4
    pub fn set_family_v4(&mut self, value: u64) {
        self.family_v4 = Some(value);
    }

    /// Gets the value of FamilyV4
    pub fn get_family_v4(&self) -> Option<&u64> {
        self.family_v4.as_ref()
    }

    /// Sets the value of FamilyV6
    pub fn set_family_v6(&mut self, value: u64) {
        self.family_v6 = Some(value);
    }

    /// Gets the value of FamilyV6
    pub fn get_family_v6(&self) -> Option<&u64> {
        self.family_v6.as_ref()
    }

    /// Sets the value of Inbound
    pub fn set_inbound(&mut self, value: u64) {
        self.inbound = Some(value);
    }

    /// Gets the value of Inbound
    pub fn get_inbound(&self) -> Option<&u64> {
        self.inbound.as_ref()
    }

    /// Sets the value of Outbound
    pub fn set_outbound(&mut self, value: u64) {
        self.outbound = Some(value);
    }

    /// Gets the value of Outbound
    pub fn get_outbound(&self) -> Option<&u64> {
        self.outbound.as_ref()
    }

    /// Sets the value of ProtocolICMP
    pub fn set_protocol_icmp(&mut self, value: u64) {
        self.protocol_icmp = Some(value);
    }

    /// Gets the value of ProtocolICMP
    pub fn get_protocol_icmp(&self) -> Option<&u64> {
        self.protocol_icmp.as_ref()
    }

    /// Sets the value of ProtocolICMP6
    pub fn set_protocol_icmp6(&mut self, value: u64) {
        self.protocol_icmp6 = Some(value);
    }

    /// Gets the value of ProtocolICMP6
    pub fn get_protocol_icmp6(&self) -> Option<&u64> {
        self.protocol_icmp6.as_ref()
    }

    /// Sets the value of ProtocolIPv4
    pub fn set_protocol_ipv4(&mut self, value: u64) {
        self.protocol_ipv4 = Some(value);
    }

    /// Gets the value of ProtocolIPv4
    pub fn get_protocol_ipv4(&self) -> Option<&u64> {
        self.protocol_ipv4.as_ref()
    }

    /// Sets the value of ProtocolIPv6
    pub fn set_protocol_ipv6(&mut self, value: u64) {
        self.protocol_ipv6 = Some(value);
    }

    /// Gets the value of ProtocolIPv6
    pub fn get_protocol_ipv6(&self) -> Option<&u64> {
        self.protocol_ipv6.as_ref()
    }

    /// Sets the value of ProtocolOther
    pub fn set_protocol_other(&mut self, value: u64) {
        self.protocol_other = Some(value);
    }

    /// Gets the value of ProtocolOther
    pub fn get_protocol_other(&self) -> Option<&u64> {
        self.protocol_other.as_ref()
    }

    /// Sets the value of ProtocolTCP
    pub fn set_protocol_tcp(&mut self, value: u64) {
        self.protocol_tcp = Some(value);
    }

    /// Gets the value of ProtocolTCP
    pub fn get_protocol_tcp(&self) -> Option<&u64> {
        self.protocol_tcp.as_ref()
    }

    /// Sets the value of ProtocolUDP
    pub fn set_protocol_udp(&mut self, value: u64) {
        self.protocol_udp = Some(value);
    }

    /// Gets the value of ProtocolUDP
    pub fn get_protocol_udp(&self) -> Option<&u64> {
        self.protocol_udp.as_ref()
    }

    /// Sets the value of ReasonClassifyCompletion
    pub fn set_reason_classify_completion(&mut self, value: u64) {
        self.reason_classify_completion = Some(value);
    }

    /// Gets the value of ReasonClassifyCompletion
    pub fn get_reason_classify_completion(&self) -> Option<&u64> {
        self.reason_classify_completion.as_ref()
    }

    /// Sets the value of ReasonEDPPolicyChanged
    pub fn set_reason_edppolicy_changed(&mut self, value: u64) {
        self.reason_edppolicy_changed = Some(value);
    }

    /// Gets the value of ReasonEDPPolicyChanged
    pub fn get_reason_edppolicy_changed(&self) -> Option<&u64> {
        self.reason_edppolicy_changed.as_ref()
    }

    /// Sets the value of ReasonIPSecPropertiesChanged
    pub fn set_reason_ipsec_properties_changed(&mut self, value: u64) {
        self.reason_ipsec_properties_changed = Some(value);
    }

    /// Gets the value of ReasonIPSecPropertiesChanged
    pub fn get_reason_ipsec_properties_changed(&self) -> Option<&u64> {
        self.reason_ipsec_properties_changed.as_ref()
    }

    /// Sets the value of ReasonMidStreamInspection
    pub fn set_reason_mid_stream_inspection(&mut self, value: u64) {
        self.reason_mid_stream_inspection = Some(value);
    }

    /// Gets the value of ReasonMidStreamInspection
    pub fn get_reason_mid_stream_inspection(&self) -> Option<&u64> {
        self.reason_mid_stream_inspection.as_ref()
    }

    /// Sets the value of ReasonNewArrivalInterface
    pub fn set_reason_new_arrival_interface(&mut self, value: u64) {
        self.reason_new_arrival_interface = Some(value);
    }

    /// Gets the value of ReasonNewArrivalInterface
    pub fn get_reason_new_arrival_interface(&self) -> Option<&u64> {
        self.reason_new_arrival_interface.as_ref()
    }

    /// Sets the value of ReasonNewInboundMCastBCastPacket
    pub fn set_reason_new_inbound_mcast_bcast_packet(&mut self, value: u64) {
        self.reason_new_inbound_mcast_bcast_packet = Some(value);
    }

    /// Gets the value of ReasonNewInboundMCastBCastPacket
    pub fn get_reason_new_inbound_mcast_bcast_packet(&self) -> Option<&u64> {
        self.reason_new_inbound_mcast_bcast_packet.as_ref()
    }

    /// Sets the value of ReasonNewNextHopInterface
    pub fn set_reason_new_next_hop_interface(&mut self, value: u64) {
        self.reason_new_next_hop_interface = Some(value);
    }

    /// Gets the value of ReasonNewNextHopInterface
    pub fn get_reason_new_next_hop_interface(&self) -> Option<&u64> {
        self.reason_new_next_hop_interface.as_ref()
    }

    /// Sets the value of ReasonPolicyChange
    pub fn set_reason_policy_change(&mut self, value: u64) {
        self.reason_policy_change = Some(value);
    }

    /// Gets the value of ReasonPolicyChange
    pub fn get_reason_policy_change(&self) -> Option<&u64> {
        self.reason_policy_change.as_ref()
    }

    /// Sets the value of ReasonProfileCrossing
    pub fn set_reason_profile_crossing(&mut self, value: u64) {
        self.reason_profile_crossing = Some(value);
    }

    /// Gets the value of ReasonProfileCrossing
    pub fn get_reason_profile_crossing(&self) -> Option<&u64> {
        self.reason_profile_crossing.as_ref()
    }

    /// Sets the value of ReasonProxyHandleChanged
    pub fn set_reason_proxy_handle_changed(&mut self, value: u64) {
        self.reason_proxy_handle_changed = Some(value);
    }

    /// Gets the value of ReasonProxyHandleChanged
    pub fn get_reason_proxy_handle_changed(&self) -> Option<&u64> {
        self.reason_proxy_handle_changed.as_ref()
    }

    /// Sets the value of ReasonSocketPropertyChanged
    pub fn set_reason_socket_property_changed(&mut self, value: u64) {
        self.reason_socket_property_changed = Some(value);
    }

    /// Gets the value of ReasonSocketPropertyChanged
    pub fn get_reason_socket_property_changed(&self) -> Option<&u64> {
        self.reason_socket_property_changed.as_ref()
    }
}

