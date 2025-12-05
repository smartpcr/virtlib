// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_VFPPortTotalOutboundDroppedNetworkPackets struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_VFPPortTotalOutboundDroppedNetworkPackets {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "TotalOutboundDroppedACLPackets")]
    pub total_outbound_dropped_aclpackets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundDroppedARPFilterPackets")]
    pub total_outbound_dropped_arpfilter_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundDroppedARPGuardPackets")]
    pub total_outbound_dropped_arpguard_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundDroppedARPLimiterPackets")]
    pub total_outbound_dropped_arplimiter_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundDroppedBlockedPackets")]
    pub total_outbound_dropped_blocked_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundDroppedBroadcastPackets")]
    pub total_outbound_dropped_broadcast_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundDroppedDHCPGuardPackets")]
    pub total_outbound_dropped_dhcpguard_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundDroppedDHCPLimiterPackets")]
    pub total_outbound_dropped_dhcplimiter_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundDroppedForwardingPackets")]
    pub total_outbound_dropped_forwarding_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundDroppedGFTCopyPackets")]
    pub total_outbound_dropped_gftcopy_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundDroppedGFTExceptionPackets")]
    pub total_outbound_dropped_gftexception_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundDroppedInvalidPackets")]
    pub total_outbound_dropped_invalid_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundDroppedInvalidRuleMatchPackets")]
    pub total_outbound_dropped_invalid_rule_match_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundDroppedIPV4SpoofingPackets")]
    pub total_outbound_dropped_ipv4_spoofing_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundDroppedIPV6SpoofingPackets")]
    pub total_outbound_dropped_ipv6_spoofing_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundDroppedMACSpoofingPackets")]
    pub total_outbound_dropped_macspoofing_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundDroppedMalformedPackets")]
    pub total_outbound_dropped_malformed_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundDroppedMonitoringPingPackets")]
    pub total_outbound_dropped_monitoring_ping_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundDroppedNonIPPackets")]
    pub total_outbound_dropped_non_ippackets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundDroppedNoResourcePackets")]
    pub total_outbound_dropped_no_resource_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundDroppedPackets")]
    pub total_outbound_dropped_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundDroppedPendingPackets")]
    pub total_outbound_dropped_pending_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundDroppedSimulationPackets")]
    pub total_outbound_dropped_simulation_packets: Option<u64>,
}

impl Win32_PerfRawData_Counters_VFPPortTotalOutboundDroppedNetworkPackets {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            total_outbound_dropped_aclpackets: None,
            total_outbound_dropped_arpfilter_packets: None,
            total_outbound_dropped_arpguard_packets: None,
            total_outbound_dropped_arplimiter_packets: None,
            total_outbound_dropped_blocked_packets: None,
            total_outbound_dropped_broadcast_packets: None,
            total_outbound_dropped_dhcpguard_packets: None,
            total_outbound_dropped_dhcplimiter_packets: None,
            total_outbound_dropped_forwarding_packets: None,
            total_outbound_dropped_gftcopy_packets: None,
            total_outbound_dropped_gftexception_packets: None,
            total_outbound_dropped_invalid_packets: None,
            total_outbound_dropped_invalid_rule_match_packets: None,
            total_outbound_dropped_ipv4_spoofing_packets: None,
            total_outbound_dropped_ipv6_spoofing_packets: None,
            total_outbound_dropped_macspoofing_packets: None,
            total_outbound_dropped_malformed_packets: None,
            total_outbound_dropped_monitoring_ping_packets: None,
            total_outbound_dropped_non_ippackets: None,
            total_outbound_dropped_no_resource_packets: None,
            total_outbound_dropped_packets: None,
            total_outbound_dropped_pending_packets: None,
            total_outbound_dropped_simulation_packets: None,
        }
    }


    /// Sets the value of TotalOutboundDroppedACLPackets
    pub fn set_total_outbound_dropped_aclpackets(&mut self, value: u64) {
        self.total_outbound_dropped_aclpackets = Some(value);
    }

    /// Gets the value of TotalOutboundDroppedACLPackets
    pub fn get_total_outbound_dropped_aclpackets(&self) -> Option<&u64> {
        self.total_outbound_dropped_aclpackets.as_ref()
    }

    /// Sets the value of TotalOutboundDroppedARPFilterPackets
    pub fn set_total_outbound_dropped_arpfilter_packets(&mut self, value: u64) {
        self.total_outbound_dropped_arpfilter_packets = Some(value);
    }

    /// Gets the value of TotalOutboundDroppedARPFilterPackets
    pub fn get_total_outbound_dropped_arpfilter_packets(&self) -> Option<&u64> {
        self.total_outbound_dropped_arpfilter_packets.as_ref()
    }

    /// Sets the value of TotalOutboundDroppedARPGuardPackets
    pub fn set_total_outbound_dropped_arpguard_packets(&mut self, value: u64) {
        self.total_outbound_dropped_arpguard_packets = Some(value);
    }

    /// Gets the value of TotalOutboundDroppedARPGuardPackets
    pub fn get_total_outbound_dropped_arpguard_packets(&self) -> Option<&u64> {
        self.total_outbound_dropped_arpguard_packets.as_ref()
    }

    /// Sets the value of TotalOutboundDroppedARPLimiterPackets
    pub fn set_total_outbound_dropped_arplimiter_packets(&mut self, value: u64) {
        self.total_outbound_dropped_arplimiter_packets = Some(value);
    }

    /// Gets the value of TotalOutboundDroppedARPLimiterPackets
    pub fn get_total_outbound_dropped_arplimiter_packets(&self) -> Option<&u64> {
        self.total_outbound_dropped_arplimiter_packets.as_ref()
    }

    /// Sets the value of TotalOutboundDroppedBlockedPackets
    pub fn set_total_outbound_dropped_blocked_packets(&mut self, value: u64) {
        self.total_outbound_dropped_blocked_packets = Some(value);
    }

    /// Gets the value of TotalOutboundDroppedBlockedPackets
    pub fn get_total_outbound_dropped_blocked_packets(&self) -> Option<&u64> {
        self.total_outbound_dropped_blocked_packets.as_ref()
    }

    /// Sets the value of TotalOutboundDroppedBroadcastPackets
    pub fn set_total_outbound_dropped_broadcast_packets(&mut self, value: u64) {
        self.total_outbound_dropped_broadcast_packets = Some(value);
    }

    /// Gets the value of TotalOutboundDroppedBroadcastPackets
    pub fn get_total_outbound_dropped_broadcast_packets(&self) -> Option<&u64> {
        self.total_outbound_dropped_broadcast_packets.as_ref()
    }

    /// Sets the value of TotalOutboundDroppedDHCPGuardPackets
    pub fn set_total_outbound_dropped_dhcpguard_packets(&mut self, value: u64) {
        self.total_outbound_dropped_dhcpguard_packets = Some(value);
    }

    /// Gets the value of TotalOutboundDroppedDHCPGuardPackets
    pub fn get_total_outbound_dropped_dhcpguard_packets(&self) -> Option<&u64> {
        self.total_outbound_dropped_dhcpguard_packets.as_ref()
    }

    /// Sets the value of TotalOutboundDroppedDHCPLimiterPackets
    pub fn set_total_outbound_dropped_dhcplimiter_packets(&mut self, value: u64) {
        self.total_outbound_dropped_dhcplimiter_packets = Some(value);
    }

    /// Gets the value of TotalOutboundDroppedDHCPLimiterPackets
    pub fn get_total_outbound_dropped_dhcplimiter_packets(&self) -> Option<&u64> {
        self.total_outbound_dropped_dhcplimiter_packets.as_ref()
    }

    /// Sets the value of TotalOutboundDroppedForwardingPackets
    pub fn set_total_outbound_dropped_forwarding_packets(&mut self, value: u64) {
        self.total_outbound_dropped_forwarding_packets = Some(value);
    }

    /// Gets the value of TotalOutboundDroppedForwardingPackets
    pub fn get_total_outbound_dropped_forwarding_packets(&self) -> Option<&u64> {
        self.total_outbound_dropped_forwarding_packets.as_ref()
    }

    /// Sets the value of TotalOutboundDroppedGFTCopyPackets
    pub fn set_total_outbound_dropped_gftcopy_packets(&mut self, value: u64) {
        self.total_outbound_dropped_gftcopy_packets = Some(value);
    }

    /// Gets the value of TotalOutboundDroppedGFTCopyPackets
    pub fn get_total_outbound_dropped_gftcopy_packets(&self) -> Option<&u64> {
        self.total_outbound_dropped_gftcopy_packets.as_ref()
    }

    /// Sets the value of TotalOutboundDroppedGFTExceptionPackets
    pub fn set_total_outbound_dropped_gftexception_packets(&mut self, value: u64) {
        self.total_outbound_dropped_gftexception_packets = Some(value);
    }

    /// Gets the value of TotalOutboundDroppedGFTExceptionPackets
    pub fn get_total_outbound_dropped_gftexception_packets(&self) -> Option<&u64> {
        self.total_outbound_dropped_gftexception_packets.as_ref()
    }

    /// Sets the value of TotalOutboundDroppedInvalidPackets
    pub fn set_total_outbound_dropped_invalid_packets(&mut self, value: u64) {
        self.total_outbound_dropped_invalid_packets = Some(value);
    }

    /// Gets the value of TotalOutboundDroppedInvalidPackets
    pub fn get_total_outbound_dropped_invalid_packets(&self) -> Option<&u64> {
        self.total_outbound_dropped_invalid_packets.as_ref()
    }

    /// Sets the value of TotalOutboundDroppedInvalidRuleMatchPackets
    pub fn set_total_outbound_dropped_invalid_rule_match_packets(&mut self, value: u64) {
        self.total_outbound_dropped_invalid_rule_match_packets = Some(value);
    }

    /// Gets the value of TotalOutboundDroppedInvalidRuleMatchPackets
    pub fn get_total_outbound_dropped_invalid_rule_match_packets(&self) -> Option<&u64> {
        self.total_outbound_dropped_invalid_rule_match_packets.as_ref()
    }

    /// Sets the value of TotalOutboundDroppedIPV4SpoofingPackets
    pub fn set_total_outbound_dropped_ipv4_spoofing_packets(&mut self, value: u64) {
        self.total_outbound_dropped_ipv4_spoofing_packets = Some(value);
    }

    /// Gets the value of TotalOutboundDroppedIPV4SpoofingPackets
    pub fn get_total_outbound_dropped_ipv4_spoofing_packets(&self) -> Option<&u64> {
        self.total_outbound_dropped_ipv4_spoofing_packets.as_ref()
    }

    /// Sets the value of TotalOutboundDroppedIPV6SpoofingPackets
    pub fn set_total_outbound_dropped_ipv6_spoofing_packets(&mut self, value: u64) {
        self.total_outbound_dropped_ipv6_spoofing_packets = Some(value);
    }

    /// Gets the value of TotalOutboundDroppedIPV6SpoofingPackets
    pub fn get_total_outbound_dropped_ipv6_spoofing_packets(&self) -> Option<&u64> {
        self.total_outbound_dropped_ipv6_spoofing_packets.as_ref()
    }

    /// Sets the value of TotalOutboundDroppedMACSpoofingPackets
    pub fn set_total_outbound_dropped_macspoofing_packets(&mut self, value: u64) {
        self.total_outbound_dropped_macspoofing_packets = Some(value);
    }

    /// Gets the value of TotalOutboundDroppedMACSpoofingPackets
    pub fn get_total_outbound_dropped_macspoofing_packets(&self) -> Option<&u64> {
        self.total_outbound_dropped_macspoofing_packets.as_ref()
    }

    /// Sets the value of TotalOutboundDroppedMalformedPackets
    pub fn set_total_outbound_dropped_malformed_packets(&mut self, value: u64) {
        self.total_outbound_dropped_malformed_packets = Some(value);
    }

    /// Gets the value of TotalOutboundDroppedMalformedPackets
    pub fn get_total_outbound_dropped_malformed_packets(&self) -> Option<&u64> {
        self.total_outbound_dropped_malformed_packets.as_ref()
    }

    /// Sets the value of TotalOutboundDroppedMonitoringPingPackets
    pub fn set_total_outbound_dropped_monitoring_ping_packets(&mut self, value: u64) {
        self.total_outbound_dropped_monitoring_ping_packets = Some(value);
    }

    /// Gets the value of TotalOutboundDroppedMonitoringPingPackets
    pub fn get_total_outbound_dropped_monitoring_ping_packets(&self) -> Option<&u64> {
        self.total_outbound_dropped_monitoring_ping_packets.as_ref()
    }

    /// Sets the value of TotalOutboundDroppedNonIPPackets
    pub fn set_total_outbound_dropped_non_ippackets(&mut self, value: u64) {
        self.total_outbound_dropped_non_ippackets = Some(value);
    }

    /// Gets the value of TotalOutboundDroppedNonIPPackets
    pub fn get_total_outbound_dropped_non_ippackets(&self) -> Option<&u64> {
        self.total_outbound_dropped_non_ippackets.as_ref()
    }

    /// Sets the value of TotalOutboundDroppedNoResourcePackets
    pub fn set_total_outbound_dropped_no_resource_packets(&mut self, value: u64) {
        self.total_outbound_dropped_no_resource_packets = Some(value);
    }

    /// Gets the value of TotalOutboundDroppedNoResourcePackets
    pub fn get_total_outbound_dropped_no_resource_packets(&self) -> Option<&u64> {
        self.total_outbound_dropped_no_resource_packets.as_ref()
    }

    /// Sets the value of TotalOutboundDroppedPackets
    pub fn set_total_outbound_dropped_packets(&mut self, value: u64) {
        self.total_outbound_dropped_packets = Some(value);
    }

    /// Gets the value of TotalOutboundDroppedPackets
    pub fn get_total_outbound_dropped_packets(&self) -> Option<&u64> {
        self.total_outbound_dropped_packets.as_ref()
    }

    /// Sets the value of TotalOutboundDroppedPendingPackets
    pub fn set_total_outbound_dropped_pending_packets(&mut self, value: u64) {
        self.total_outbound_dropped_pending_packets = Some(value);
    }

    /// Gets the value of TotalOutboundDroppedPendingPackets
    pub fn get_total_outbound_dropped_pending_packets(&self) -> Option<&u64> {
        self.total_outbound_dropped_pending_packets.as_ref()
    }

    /// Sets the value of TotalOutboundDroppedSimulationPackets
    pub fn set_total_outbound_dropped_simulation_packets(&mut self, value: u64) {
        self.total_outbound_dropped_simulation_packets = Some(value);
    }

    /// Gets the value of TotalOutboundDroppedSimulationPackets
    pub fn get_total_outbound_dropped_simulation_packets(&self) -> Option<&u64> {
        self.total_outbound_dropped_simulation_packets.as_ref()
    }
}

