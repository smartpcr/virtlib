// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_VFPPortTotalInboundDroppedNetworkPackets struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_VFPPortTotalInboundDroppedNetworkPackets {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "TotalInboundDroppedACLPackets")]
    pub total_inbound_dropped_aclpackets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundDroppedARPFilterPackets")]
    pub total_inbound_dropped_arpfilter_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundDroppedARPGuardPackets")]
    pub total_inbound_dropped_arpguard_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundDroppedARPLimiterPackets")]
    pub total_inbound_dropped_arplimiter_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundDroppedBlockedPackets")]
    pub total_inbound_dropped_blocked_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundDroppedBroadcastPackets")]
    pub total_inbound_dropped_broadcast_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundDroppedDHCPGuardPackets")]
    pub total_inbound_dropped_dhcpguard_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundDroppedDHCPLimiterPackets")]
    pub total_inbound_dropped_dhcplimiter_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundDroppedForwardingPackets")]
    pub total_inbound_dropped_forwarding_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundDroppedGFTCopyPackets")]
    pub total_inbound_dropped_gftcopy_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundDroppedGFTExceptionPackets")]
    pub total_inbound_dropped_gftexception_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundDroppedInvalidPackets")]
    pub total_inbound_dropped_invalid_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundDroppedInvalidRuleMatchPackets")]
    pub total_inbound_dropped_invalid_rule_match_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundDroppedIPV4SpoofingPackets")]
    pub total_inbound_dropped_ipv4_spoofing_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundDroppedIPV6SpoofingPackets")]
    pub total_inbound_dropped_ipv6_spoofing_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundDroppedMACSpoofingPackets")]
    pub total_inbound_dropped_macspoofing_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundDroppedMalformedPackets")]
    pub total_inbound_dropped_malformed_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundDroppedMonitoringPingPackets")]
    pub total_inbound_dropped_monitoring_ping_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundDroppedNonIPPackets")]
    pub total_inbound_dropped_non_ippackets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundDroppedNoResourcePackets")]
    pub total_inbound_dropped_no_resource_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundDroppedPackets")]
    pub total_inbound_dropped_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundDroppedPendingPackets")]
    pub total_inbound_dropped_pending_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundDroppedSimulationPackets")]
    pub total_inbound_dropped_simulation_packets: Option<u64>,
}

impl Win32_PerfRawData_Counters_VFPPortTotalInboundDroppedNetworkPackets {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            total_inbound_dropped_aclpackets: None,
            total_inbound_dropped_arpfilter_packets: None,
            total_inbound_dropped_arpguard_packets: None,
            total_inbound_dropped_arplimiter_packets: None,
            total_inbound_dropped_blocked_packets: None,
            total_inbound_dropped_broadcast_packets: None,
            total_inbound_dropped_dhcpguard_packets: None,
            total_inbound_dropped_dhcplimiter_packets: None,
            total_inbound_dropped_forwarding_packets: None,
            total_inbound_dropped_gftcopy_packets: None,
            total_inbound_dropped_gftexception_packets: None,
            total_inbound_dropped_invalid_packets: None,
            total_inbound_dropped_invalid_rule_match_packets: None,
            total_inbound_dropped_ipv4_spoofing_packets: None,
            total_inbound_dropped_ipv6_spoofing_packets: None,
            total_inbound_dropped_macspoofing_packets: None,
            total_inbound_dropped_malformed_packets: None,
            total_inbound_dropped_monitoring_ping_packets: None,
            total_inbound_dropped_non_ippackets: None,
            total_inbound_dropped_no_resource_packets: None,
            total_inbound_dropped_packets: None,
            total_inbound_dropped_pending_packets: None,
            total_inbound_dropped_simulation_packets: None,
        }
    }


    /// Sets the value of TotalInboundDroppedACLPackets
    pub fn set_total_inbound_dropped_aclpackets(&mut self, value: u64) {
        self.total_inbound_dropped_aclpackets = Some(value);
    }

    /// Gets the value of TotalInboundDroppedACLPackets
    pub fn get_total_inbound_dropped_aclpackets(&self) -> Option<&u64> {
        self.total_inbound_dropped_aclpackets.as_ref()
    }

    /// Sets the value of TotalInboundDroppedARPFilterPackets
    pub fn set_total_inbound_dropped_arpfilter_packets(&mut self, value: u64) {
        self.total_inbound_dropped_arpfilter_packets = Some(value);
    }

    /// Gets the value of TotalInboundDroppedARPFilterPackets
    pub fn get_total_inbound_dropped_arpfilter_packets(&self) -> Option<&u64> {
        self.total_inbound_dropped_arpfilter_packets.as_ref()
    }

    /// Sets the value of TotalInboundDroppedARPGuardPackets
    pub fn set_total_inbound_dropped_arpguard_packets(&mut self, value: u64) {
        self.total_inbound_dropped_arpguard_packets = Some(value);
    }

    /// Gets the value of TotalInboundDroppedARPGuardPackets
    pub fn get_total_inbound_dropped_arpguard_packets(&self) -> Option<&u64> {
        self.total_inbound_dropped_arpguard_packets.as_ref()
    }

    /// Sets the value of TotalInboundDroppedARPLimiterPackets
    pub fn set_total_inbound_dropped_arplimiter_packets(&mut self, value: u64) {
        self.total_inbound_dropped_arplimiter_packets = Some(value);
    }

    /// Gets the value of TotalInboundDroppedARPLimiterPackets
    pub fn get_total_inbound_dropped_arplimiter_packets(&self) -> Option<&u64> {
        self.total_inbound_dropped_arplimiter_packets.as_ref()
    }

    /// Sets the value of TotalInboundDroppedBlockedPackets
    pub fn set_total_inbound_dropped_blocked_packets(&mut self, value: u64) {
        self.total_inbound_dropped_blocked_packets = Some(value);
    }

    /// Gets the value of TotalInboundDroppedBlockedPackets
    pub fn get_total_inbound_dropped_blocked_packets(&self) -> Option<&u64> {
        self.total_inbound_dropped_blocked_packets.as_ref()
    }

    /// Sets the value of TotalInboundDroppedBroadcastPackets
    pub fn set_total_inbound_dropped_broadcast_packets(&mut self, value: u64) {
        self.total_inbound_dropped_broadcast_packets = Some(value);
    }

    /// Gets the value of TotalInboundDroppedBroadcastPackets
    pub fn get_total_inbound_dropped_broadcast_packets(&self) -> Option<&u64> {
        self.total_inbound_dropped_broadcast_packets.as_ref()
    }

    /// Sets the value of TotalInboundDroppedDHCPGuardPackets
    pub fn set_total_inbound_dropped_dhcpguard_packets(&mut self, value: u64) {
        self.total_inbound_dropped_dhcpguard_packets = Some(value);
    }

    /// Gets the value of TotalInboundDroppedDHCPGuardPackets
    pub fn get_total_inbound_dropped_dhcpguard_packets(&self) -> Option<&u64> {
        self.total_inbound_dropped_dhcpguard_packets.as_ref()
    }

    /// Sets the value of TotalInboundDroppedDHCPLimiterPackets
    pub fn set_total_inbound_dropped_dhcplimiter_packets(&mut self, value: u64) {
        self.total_inbound_dropped_dhcplimiter_packets = Some(value);
    }

    /// Gets the value of TotalInboundDroppedDHCPLimiterPackets
    pub fn get_total_inbound_dropped_dhcplimiter_packets(&self) -> Option<&u64> {
        self.total_inbound_dropped_dhcplimiter_packets.as_ref()
    }

    /// Sets the value of TotalInboundDroppedForwardingPackets
    pub fn set_total_inbound_dropped_forwarding_packets(&mut self, value: u64) {
        self.total_inbound_dropped_forwarding_packets = Some(value);
    }

    /// Gets the value of TotalInboundDroppedForwardingPackets
    pub fn get_total_inbound_dropped_forwarding_packets(&self) -> Option<&u64> {
        self.total_inbound_dropped_forwarding_packets.as_ref()
    }

    /// Sets the value of TotalInboundDroppedGFTCopyPackets
    pub fn set_total_inbound_dropped_gftcopy_packets(&mut self, value: u64) {
        self.total_inbound_dropped_gftcopy_packets = Some(value);
    }

    /// Gets the value of TotalInboundDroppedGFTCopyPackets
    pub fn get_total_inbound_dropped_gftcopy_packets(&self) -> Option<&u64> {
        self.total_inbound_dropped_gftcopy_packets.as_ref()
    }

    /// Sets the value of TotalInboundDroppedGFTExceptionPackets
    pub fn set_total_inbound_dropped_gftexception_packets(&mut self, value: u64) {
        self.total_inbound_dropped_gftexception_packets = Some(value);
    }

    /// Gets the value of TotalInboundDroppedGFTExceptionPackets
    pub fn get_total_inbound_dropped_gftexception_packets(&self) -> Option<&u64> {
        self.total_inbound_dropped_gftexception_packets.as_ref()
    }

    /// Sets the value of TotalInboundDroppedInvalidPackets
    pub fn set_total_inbound_dropped_invalid_packets(&mut self, value: u64) {
        self.total_inbound_dropped_invalid_packets = Some(value);
    }

    /// Gets the value of TotalInboundDroppedInvalidPackets
    pub fn get_total_inbound_dropped_invalid_packets(&self) -> Option<&u64> {
        self.total_inbound_dropped_invalid_packets.as_ref()
    }

    /// Sets the value of TotalInboundDroppedInvalidRuleMatchPackets
    pub fn set_total_inbound_dropped_invalid_rule_match_packets(&mut self, value: u64) {
        self.total_inbound_dropped_invalid_rule_match_packets = Some(value);
    }

    /// Gets the value of TotalInboundDroppedInvalidRuleMatchPackets
    pub fn get_total_inbound_dropped_invalid_rule_match_packets(&self) -> Option<&u64> {
        self.total_inbound_dropped_invalid_rule_match_packets.as_ref()
    }

    /// Sets the value of TotalInboundDroppedIPV4SpoofingPackets
    pub fn set_total_inbound_dropped_ipv4_spoofing_packets(&mut self, value: u64) {
        self.total_inbound_dropped_ipv4_spoofing_packets = Some(value);
    }

    /// Gets the value of TotalInboundDroppedIPV4SpoofingPackets
    pub fn get_total_inbound_dropped_ipv4_spoofing_packets(&self) -> Option<&u64> {
        self.total_inbound_dropped_ipv4_spoofing_packets.as_ref()
    }

    /// Sets the value of TotalInboundDroppedIPV6SpoofingPackets
    pub fn set_total_inbound_dropped_ipv6_spoofing_packets(&mut self, value: u64) {
        self.total_inbound_dropped_ipv6_spoofing_packets = Some(value);
    }

    /// Gets the value of TotalInboundDroppedIPV6SpoofingPackets
    pub fn get_total_inbound_dropped_ipv6_spoofing_packets(&self) -> Option<&u64> {
        self.total_inbound_dropped_ipv6_spoofing_packets.as_ref()
    }

    /// Sets the value of TotalInboundDroppedMACSpoofingPackets
    pub fn set_total_inbound_dropped_macspoofing_packets(&mut self, value: u64) {
        self.total_inbound_dropped_macspoofing_packets = Some(value);
    }

    /// Gets the value of TotalInboundDroppedMACSpoofingPackets
    pub fn get_total_inbound_dropped_macspoofing_packets(&self) -> Option<&u64> {
        self.total_inbound_dropped_macspoofing_packets.as_ref()
    }

    /// Sets the value of TotalInboundDroppedMalformedPackets
    pub fn set_total_inbound_dropped_malformed_packets(&mut self, value: u64) {
        self.total_inbound_dropped_malformed_packets = Some(value);
    }

    /// Gets the value of TotalInboundDroppedMalformedPackets
    pub fn get_total_inbound_dropped_malformed_packets(&self) -> Option<&u64> {
        self.total_inbound_dropped_malformed_packets.as_ref()
    }

    /// Sets the value of TotalInboundDroppedMonitoringPingPackets
    pub fn set_total_inbound_dropped_monitoring_ping_packets(&mut self, value: u64) {
        self.total_inbound_dropped_monitoring_ping_packets = Some(value);
    }

    /// Gets the value of TotalInboundDroppedMonitoringPingPackets
    pub fn get_total_inbound_dropped_monitoring_ping_packets(&self) -> Option<&u64> {
        self.total_inbound_dropped_monitoring_ping_packets.as_ref()
    }

    /// Sets the value of TotalInboundDroppedNonIPPackets
    pub fn set_total_inbound_dropped_non_ippackets(&mut self, value: u64) {
        self.total_inbound_dropped_non_ippackets = Some(value);
    }

    /// Gets the value of TotalInboundDroppedNonIPPackets
    pub fn get_total_inbound_dropped_non_ippackets(&self) -> Option<&u64> {
        self.total_inbound_dropped_non_ippackets.as_ref()
    }

    /// Sets the value of TotalInboundDroppedNoResourcePackets
    pub fn set_total_inbound_dropped_no_resource_packets(&mut self, value: u64) {
        self.total_inbound_dropped_no_resource_packets = Some(value);
    }

    /// Gets the value of TotalInboundDroppedNoResourcePackets
    pub fn get_total_inbound_dropped_no_resource_packets(&self) -> Option<&u64> {
        self.total_inbound_dropped_no_resource_packets.as_ref()
    }

    /// Sets the value of TotalInboundDroppedPackets
    pub fn set_total_inbound_dropped_packets(&mut self, value: u64) {
        self.total_inbound_dropped_packets = Some(value);
    }

    /// Gets the value of TotalInboundDroppedPackets
    pub fn get_total_inbound_dropped_packets(&self) -> Option<&u64> {
        self.total_inbound_dropped_packets.as_ref()
    }

    /// Sets the value of TotalInboundDroppedPendingPackets
    pub fn set_total_inbound_dropped_pending_packets(&mut self, value: u64) {
        self.total_inbound_dropped_pending_packets = Some(value);
    }

    /// Gets the value of TotalInboundDroppedPendingPackets
    pub fn get_total_inbound_dropped_pending_packets(&self) -> Option<&u64> {
        self.total_inbound_dropped_pending_packets.as_ref()
    }

    /// Sets the value of TotalInboundDroppedSimulationPackets
    pub fn set_total_inbound_dropped_simulation_packets(&mut self, value: u64) {
        self.total_inbound_dropped_simulation_packets = Some(value);
    }

    /// Gets the value of TotalInboundDroppedSimulationPackets
    pub fn get_total_inbound_dropped_simulation_packets(&self) -> Option<&u64> {
        self.total_inbound_dropped_simulation_packets.as_ref()
    }
}

