// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_IPsecDoSProtection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_IPsecDoSProtection {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "CurrentStateEntries")]
    pub current_state_entries: Option<u64>,

/// 
    #[serde(rename = "InboundAllowedDefaultBlockExemptPackets")]
    pub inbound_allowed_default_block_exempt_packets: Option<u64>,

/// 
    #[serde(rename = "InboundAllowedDefaultBlockExemptPacketsPersec")]
    pub inbound_allowed_default_block_exempt_packets_persec: Option<u32>,

/// 
    #[serde(rename = "InboundAllowedFilterExemptIPv6Packets")]
    pub inbound_allowed_filter_exempt_ipv6_packets: Option<u64>,

/// 
    #[serde(rename = "InboundAllowedFilterExemptIPv6PacketsPersec")]
    pub inbound_allowed_filter_exempt_ipv6_packets_persec: Option<u32>,

/// 
    #[serde(rename = "InboundAllowedICMPv6Packets")]
    pub inbound_allowed_icmpv6_packets: Option<u64>,

/// 
    #[serde(rename = "InboundAllowedICMPv6PacketsPersec")]
    pub inbound_allowed_icmpv6_packets_persec: Option<u32>,

/// 
    #[serde(rename = "InboundAllowedIPv6IPsecAuthenticatedPackets")]
    pub inbound_allowed_ipv6_ipsec_authenticated_packets: Option<u64>,

/// 
    #[serde(rename = "InboundAllowedIPv6IPsecAuthenticatedPacketsPersec")]
    pub inbound_allowed_ipv6_ipsec_authenticated_packets_persec: Option<u32>,

/// 
    #[serde(rename = "InboundAllowedIPv6IPsecUnauthenticatedPackets")]
    pub inbound_allowed_ipv6_ipsec_unauthenticated_packets: Option<u64>,

/// 
    #[serde(rename = "InboundAllowedIPv6IPsecUnauthenticatedPacketsPersec")]
    pub inbound_allowed_ipv6_ipsec_unauthenticated_packets_persec: Option<u32>,

/// 
    #[serde(rename = "InboundDiscardedDefaultBlockPackets")]
    pub inbound_discarded_default_block_packets: Option<u64>,

/// 
    #[serde(rename = "InboundDiscardedDefaultBlockPacketsPersec")]
    pub inbound_discarded_default_block_packets_persec: Option<u32>,

/// 
    #[serde(rename = "InboundDiscardedFilterBlockIPv6Packets")]
    pub inbound_discarded_filter_block_ipv6_packets: Option<u64>,

/// 
    #[serde(rename = "InboundDiscardedFilterBlockIPv6PacketsPersec")]
    pub inbound_discarded_filter_block_ipv6_packets_persec: Option<u32>,

/// 
    #[serde(rename = "InboundDiscardedPackets")]
    pub inbound_discarded_packets: Option<u64>,

/// 
    #[serde(rename = "InboundDiscardedPacketsPersec")]
    pub inbound_discarded_packets_persec: Option<u32>,

/// 
    #[serde(rename = "InboundOtherDiscardedIPv6IPsecAuthenticatedPackets")]
    pub inbound_other_discarded_ipv6_ipsec_authenticated_packets: Option<u64>,

/// 
    #[serde(rename = "InboundOtherDiscardedIPv6IPsecAuthenticatedPacketsPersec")]
    pub inbound_other_discarded_ipv6_ipsec_authenticated_packets_persec: Option<u32>,

/// 
    #[serde(rename = "InboundOtherDiscardedIPv6IPsecUnauthenticatedPackets")]
    pub inbound_other_discarded_ipv6_ipsec_unauthenticated_packets: Option<u64>,

/// 
    #[serde(rename = "InboundOtherDiscardedIPv6IPsecUnauthenticatedPacketsPersec")]
    pub inbound_other_discarded_ipv6_ipsec_unauthenticated_packets_persec: Option<u32>,

/// 
    #[serde(rename = "InboundPerIPRateLimitDiscardedIPv6IPsecUnauthenticatedPackets")]
    pub inbound_per_iprate_limit_discarded_ipv6_ipsec_unauthenticated_packets: Option<u64>,

/// 
    #[serde(rename = "InboundPerIPRateLimitDiscardedIPv6IPsecUnauthenticatedPacketsPersec")]
    pub inbound_per_iprate_limit_discarded_ipv6_ipsec_unauthenticated_packets_persec: Option<u32>,

/// 
    #[serde(rename = "InboundRateLimitDiscardedDefaultBlockExemptPackets")]
    pub inbound_rate_limit_discarded_default_block_exempt_packets: Option<u64>,

/// 
    #[serde(rename = "InboundRateLimitDiscardedDefaultBlockExemptPacketsPersec")]
    pub inbound_rate_limit_discarded_default_block_exempt_packets_persec: Option<u32>,

/// 
    #[serde(rename = "InboundRateLimitDiscardedFilterExemptIPv6Packets")]
    pub inbound_rate_limit_discarded_filter_exempt_ipv6_packets: Option<u64>,

/// 
    #[serde(rename = "InboundRateLimitDiscardedFilterExemptIPv6PacketsPersec")]
    pub inbound_rate_limit_discarded_filter_exempt_ipv6_packets_persec: Option<u32>,

/// 
    #[serde(rename = "InboundRateLimitDiscardedICMPv6Packets")]
    pub inbound_rate_limit_discarded_icmpv6_packets: Option<u64>,

/// 
    #[serde(rename = "InboundRateLimitDiscardedICMPv6PacketsPersec")]
    pub inbound_rate_limit_discarded_icmpv6_packets_persec: Option<u32>,

/// 
    #[serde(rename = "InboundRateLimitDiscardedIPv6IPsecAuthenticatedPackets")]
    pub inbound_rate_limit_discarded_ipv6_ipsec_authenticated_packets: Option<u64>,

/// 
    #[serde(rename = "InboundRateLimitDiscardedIPv6IPsecAuthenticatedPacketsPersec")]
    pub inbound_rate_limit_discarded_ipv6_ipsec_authenticated_packets_persec: Option<u32>,

/// 
    #[serde(rename = "InboundRateLimitDiscardedIPv6IPsecUnauthenticatedPackets")]
    pub inbound_rate_limit_discarded_ipv6_ipsec_unauthenticated_packets: Option<u64>,

/// 
    #[serde(rename = "InboundRateLimitDiscardedIPv6IPsecUnauthenticatedPacketsPersec")]
    pub inbound_rate_limit_discarded_ipv6_ipsec_unauthenticated_packets_persec: Option<u32>,

/// 
    #[serde(rename = "PerIPRateLimitQueues")]
    pub per_iprate_limit_queues: Option<u64>,

/// 
    #[serde(rename = "StateEntries")]
    pub state_entries: Option<u64>,

/// 
    #[serde(rename = "StateEntriesPersec")]
    pub state_entries_persec: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_IPsecDoSProtection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            current_state_entries: None,
            inbound_allowed_default_block_exempt_packets: None,
            inbound_allowed_default_block_exempt_packets_persec: None,
            inbound_allowed_filter_exempt_ipv6_packets: None,
            inbound_allowed_filter_exempt_ipv6_packets_persec: None,
            inbound_allowed_icmpv6_packets: None,
            inbound_allowed_icmpv6_packets_persec: None,
            inbound_allowed_ipv6_ipsec_authenticated_packets: None,
            inbound_allowed_ipv6_ipsec_authenticated_packets_persec: None,
            inbound_allowed_ipv6_ipsec_unauthenticated_packets: None,
            inbound_allowed_ipv6_ipsec_unauthenticated_packets_persec: None,
            inbound_discarded_default_block_packets: None,
            inbound_discarded_default_block_packets_persec: None,
            inbound_discarded_filter_block_ipv6_packets: None,
            inbound_discarded_filter_block_ipv6_packets_persec: None,
            inbound_discarded_packets: None,
            inbound_discarded_packets_persec: None,
            inbound_other_discarded_ipv6_ipsec_authenticated_packets: None,
            inbound_other_discarded_ipv6_ipsec_authenticated_packets_persec: None,
            inbound_other_discarded_ipv6_ipsec_unauthenticated_packets: None,
            inbound_other_discarded_ipv6_ipsec_unauthenticated_packets_persec: None,
            inbound_per_iprate_limit_discarded_ipv6_ipsec_unauthenticated_packets: None,
            inbound_per_iprate_limit_discarded_ipv6_ipsec_unauthenticated_packets_persec: None,
            inbound_rate_limit_discarded_default_block_exempt_packets: None,
            inbound_rate_limit_discarded_default_block_exempt_packets_persec: None,
            inbound_rate_limit_discarded_filter_exempt_ipv6_packets: None,
            inbound_rate_limit_discarded_filter_exempt_ipv6_packets_persec: None,
            inbound_rate_limit_discarded_icmpv6_packets: None,
            inbound_rate_limit_discarded_icmpv6_packets_persec: None,
            inbound_rate_limit_discarded_ipv6_ipsec_authenticated_packets: None,
            inbound_rate_limit_discarded_ipv6_ipsec_authenticated_packets_persec: None,
            inbound_rate_limit_discarded_ipv6_ipsec_unauthenticated_packets: None,
            inbound_rate_limit_discarded_ipv6_ipsec_unauthenticated_packets_persec: None,
            per_iprate_limit_queues: None,
            state_entries: None,
            state_entries_persec: None,
        }
    }


    /// Sets the value of CurrentStateEntries
    pub fn set_current_state_entries(&mut self, value: u64) {
        self.current_state_entries = Some(value);
    }

    /// Gets the value of CurrentStateEntries
    pub fn get_current_state_entries(&self) -> Option<&u64> {
        self.current_state_entries.as_ref()
    }

    /// Sets the value of InboundAllowedDefaultBlockExemptPackets
    pub fn set_inbound_allowed_default_block_exempt_packets(&mut self, value: u64) {
        self.inbound_allowed_default_block_exempt_packets = Some(value);
    }

    /// Gets the value of InboundAllowedDefaultBlockExemptPackets
    pub fn get_inbound_allowed_default_block_exempt_packets(&self) -> Option<&u64> {
        self.inbound_allowed_default_block_exempt_packets.as_ref()
    }

    /// Sets the value of InboundAllowedDefaultBlockExemptPacketsPersec
    pub fn set_inbound_allowed_default_block_exempt_packets_persec(&mut self, value: u32) {
        self.inbound_allowed_default_block_exempt_packets_persec = Some(value);
    }

    /// Gets the value of InboundAllowedDefaultBlockExemptPacketsPersec
    pub fn get_inbound_allowed_default_block_exempt_packets_persec(&self) -> Option<&u32> {
        self.inbound_allowed_default_block_exempt_packets_persec.as_ref()
    }

    /// Sets the value of InboundAllowedFilterExemptIPv6Packets
    pub fn set_inbound_allowed_filter_exempt_ipv6_packets(&mut self, value: u64) {
        self.inbound_allowed_filter_exempt_ipv6_packets = Some(value);
    }

    /// Gets the value of InboundAllowedFilterExemptIPv6Packets
    pub fn get_inbound_allowed_filter_exempt_ipv6_packets(&self) -> Option<&u64> {
        self.inbound_allowed_filter_exempt_ipv6_packets.as_ref()
    }

    /// Sets the value of InboundAllowedFilterExemptIPv6PacketsPersec
    pub fn set_inbound_allowed_filter_exempt_ipv6_packets_persec(&mut self, value: u32) {
        self.inbound_allowed_filter_exempt_ipv6_packets_persec = Some(value);
    }

    /// Gets the value of InboundAllowedFilterExemptIPv6PacketsPersec
    pub fn get_inbound_allowed_filter_exempt_ipv6_packets_persec(&self) -> Option<&u32> {
        self.inbound_allowed_filter_exempt_ipv6_packets_persec.as_ref()
    }

    /// Sets the value of InboundAllowedICMPv6Packets
    pub fn set_inbound_allowed_icmpv6_packets(&mut self, value: u64) {
        self.inbound_allowed_icmpv6_packets = Some(value);
    }

    /// Gets the value of InboundAllowedICMPv6Packets
    pub fn get_inbound_allowed_icmpv6_packets(&self) -> Option<&u64> {
        self.inbound_allowed_icmpv6_packets.as_ref()
    }

    /// Sets the value of InboundAllowedICMPv6PacketsPersec
    pub fn set_inbound_allowed_icmpv6_packets_persec(&mut self, value: u32) {
        self.inbound_allowed_icmpv6_packets_persec = Some(value);
    }

    /// Gets the value of InboundAllowedICMPv6PacketsPersec
    pub fn get_inbound_allowed_icmpv6_packets_persec(&self) -> Option<&u32> {
        self.inbound_allowed_icmpv6_packets_persec.as_ref()
    }

    /// Sets the value of InboundAllowedIPv6IPsecAuthenticatedPackets
    pub fn set_inbound_allowed_ipv6_ipsec_authenticated_packets(&mut self, value: u64) {
        self.inbound_allowed_ipv6_ipsec_authenticated_packets = Some(value);
    }

    /// Gets the value of InboundAllowedIPv6IPsecAuthenticatedPackets
    pub fn get_inbound_allowed_ipv6_ipsec_authenticated_packets(&self) -> Option<&u64> {
        self.inbound_allowed_ipv6_ipsec_authenticated_packets.as_ref()
    }

    /// Sets the value of InboundAllowedIPv6IPsecAuthenticatedPacketsPersec
    pub fn set_inbound_allowed_ipv6_ipsec_authenticated_packets_persec(&mut self, value: u32) {
        self.inbound_allowed_ipv6_ipsec_authenticated_packets_persec = Some(value);
    }

    /// Gets the value of InboundAllowedIPv6IPsecAuthenticatedPacketsPersec
    pub fn get_inbound_allowed_ipv6_ipsec_authenticated_packets_persec(&self) -> Option<&u32> {
        self.inbound_allowed_ipv6_ipsec_authenticated_packets_persec.as_ref()
    }

    /// Sets the value of InboundAllowedIPv6IPsecUnauthenticatedPackets
    pub fn set_inbound_allowed_ipv6_ipsec_unauthenticated_packets(&mut self, value: u64) {
        self.inbound_allowed_ipv6_ipsec_unauthenticated_packets = Some(value);
    }

    /// Gets the value of InboundAllowedIPv6IPsecUnauthenticatedPackets
    pub fn get_inbound_allowed_ipv6_ipsec_unauthenticated_packets(&self) -> Option<&u64> {
        self.inbound_allowed_ipv6_ipsec_unauthenticated_packets.as_ref()
    }

    /// Sets the value of InboundAllowedIPv6IPsecUnauthenticatedPacketsPersec
    pub fn set_inbound_allowed_ipv6_ipsec_unauthenticated_packets_persec(&mut self, value: u32) {
        self.inbound_allowed_ipv6_ipsec_unauthenticated_packets_persec = Some(value);
    }

    /// Gets the value of InboundAllowedIPv6IPsecUnauthenticatedPacketsPersec
    pub fn get_inbound_allowed_ipv6_ipsec_unauthenticated_packets_persec(&self) -> Option<&u32> {
        self.inbound_allowed_ipv6_ipsec_unauthenticated_packets_persec.as_ref()
    }

    /// Sets the value of InboundDiscardedDefaultBlockPackets
    pub fn set_inbound_discarded_default_block_packets(&mut self, value: u64) {
        self.inbound_discarded_default_block_packets = Some(value);
    }

    /// Gets the value of InboundDiscardedDefaultBlockPackets
    pub fn get_inbound_discarded_default_block_packets(&self) -> Option<&u64> {
        self.inbound_discarded_default_block_packets.as_ref()
    }

    /// Sets the value of InboundDiscardedDefaultBlockPacketsPersec
    pub fn set_inbound_discarded_default_block_packets_persec(&mut self, value: u32) {
        self.inbound_discarded_default_block_packets_persec = Some(value);
    }

    /// Gets the value of InboundDiscardedDefaultBlockPacketsPersec
    pub fn get_inbound_discarded_default_block_packets_persec(&self) -> Option<&u32> {
        self.inbound_discarded_default_block_packets_persec.as_ref()
    }

    /// Sets the value of InboundDiscardedFilterBlockIPv6Packets
    pub fn set_inbound_discarded_filter_block_ipv6_packets(&mut self, value: u64) {
        self.inbound_discarded_filter_block_ipv6_packets = Some(value);
    }

    /// Gets the value of InboundDiscardedFilterBlockIPv6Packets
    pub fn get_inbound_discarded_filter_block_ipv6_packets(&self) -> Option<&u64> {
        self.inbound_discarded_filter_block_ipv6_packets.as_ref()
    }

    /// Sets the value of InboundDiscardedFilterBlockIPv6PacketsPersec
    pub fn set_inbound_discarded_filter_block_ipv6_packets_persec(&mut self, value: u32) {
        self.inbound_discarded_filter_block_ipv6_packets_persec = Some(value);
    }

    /// Gets the value of InboundDiscardedFilterBlockIPv6PacketsPersec
    pub fn get_inbound_discarded_filter_block_ipv6_packets_persec(&self) -> Option<&u32> {
        self.inbound_discarded_filter_block_ipv6_packets_persec.as_ref()
    }

    /// Sets the value of InboundDiscardedPackets
    pub fn set_inbound_discarded_packets(&mut self, value: u64) {
        self.inbound_discarded_packets = Some(value);
    }

    /// Gets the value of InboundDiscardedPackets
    pub fn get_inbound_discarded_packets(&self) -> Option<&u64> {
        self.inbound_discarded_packets.as_ref()
    }

    /// Sets the value of InboundDiscardedPacketsPersec
    pub fn set_inbound_discarded_packets_persec(&mut self, value: u32) {
        self.inbound_discarded_packets_persec = Some(value);
    }

    /// Gets the value of InboundDiscardedPacketsPersec
    pub fn get_inbound_discarded_packets_persec(&self) -> Option<&u32> {
        self.inbound_discarded_packets_persec.as_ref()
    }

    /// Sets the value of InboundOtherDiscardedIPv6IPsecAuthenticatedPackets
    pub fn set_inbound_other_discarded_ipv6_ipsec_authenticated_packets(&mut self, value: u64) {
        self.inbound_other_discarded_ipv6_ipsec_authenticated_packets = Some(value);
    }

    /// Gets the value of InboundOtherDiscardedIPv6IPsecAuthenticatedPackets
    pub fn get_inbound_other_discarded_ipv6_ipsec_authenticated_packets(&self) -> Option<&u64> {
        self.inbound_other_discarded_ipv6_ipsec_authenticated_packets.as_ref()
    }

    /// Sets the value of InboundOtherDiscardedIPv6IPsecAuthenticatedPacketsPersec
    pub fn set_inbound_other_discarded_ipv6_ipsec_authenticated_packets_persec(&mut self, value: u32) {
        self.inbound_other_discarded_ipv6_ipsec_authenticated_packets_persec = Some(value);
    }

    /// Gets the value of InboundOtherDiscardedIPv6IPsecAuthenticatedPacketsPersec
    pub fn get_inbound_other_discarded_ipv6_ipsec_authenticated_packets_persec(&self) -> Option<&u32> {
        self.inbound_other_discarded_ipv6_ipsec_authenticated_packets_persec.as_ref()
    }

    /// Sets the value of InboundOtherDiscardedIPv6IPsecUnauthenticatedPackets
    pub fn set_inbound_other_discarded_ipv6_ipsec_unauthenticated_packets(&mut self, value: u64) {
        self.inbound_other_discarded_ipv6_ipsec_unauthenticated_packets = Some(value);
    }

    /// Gets the value of InboundOtherDiscardedIPv6IPsecUnauthenticatedPackets
    pub fn get_inbound_other_discarded_ipv6_ipsec_unauthenticated_packets(&self) -> Option<&u64> {
        self.inbound_other_discarded_ipv6_ipsec_unauthenticated_packets.as_ref()
    }

    /// Sets the value of InboundOtherDiscardedIPv6IPsecUnauthenticatedPacketsPersec
    pub fn set_inbound_other_discarded_ipv6_ipsec_unauthenticated_packets_persec(&mut self, value: u32) {
        self.inbound_other_discarded_ipv6_ipsec_unauthenticated_packets_persec = Some(value);
    }

    /// Gets the value of InboundOtherDiscardedIPv6IPsecUnauthenticatedPacketsPersec
    pub fn get_inbound_other_discarded_ipv6_ipsec_unauthenticated_packets_persec(&self) -> Option<&u32> {
        self.inbound_other_discarded_ipv6_ipsec_unauthenticated_packets_persec.as_ref()
    }

    /// Sets the value of InboundPerIPRateLimitDiscardedIPv6IPsecUnauthenticatedPackets
    pub fn set_inbound_per_iprate_limit_discarded_ipv6_ipsec_unauthenticated_packets(&mut self, value: u64) {
        self.inbound_per_iprate_limit_discarded_ipv6_ipsec_unauthenticated_packets = Some(value);
    }

    /// Gets the value of InboundPerIPRateLimitDiscardedIPv6IPsecUnauthenticatedPackets
    pub fn get_inbound_per_iprate_limit_discarded_ipv6_ipsec_unauthenticated_packets(&self) -> Option<&u64> {
        self.inbound_per_iprate_limit_discarded_ipv6_ipsec_unauthenticated_packets.as_ref()
    }

    /// Sets the value of InboundPerIPRateLimitDiscardedIPv6IPsecUnauthenticatedPacketsPersec
    pub fn set_inbound_per_iprate_limit_discarded_ipv6_ipsec_unauthenticated_packets_persec(&mut self, value: u32) {
        self.inbound_per_iprate_limit_discarded_ipv6_ipsec_unauthenticated_packets_persec = Some(value);
    }

    /// Gets the value of InboundPerIPRateLimitDiscardedIPv6IPsecUnauthenticatedPacketsPersec
    pub fn get_inbound_per_iprate_limit_discarded_ipv6_ipsec_unauthenticated_packets_persec(&self) -> Option<&u32> {
        self.inbound_per_iprate_limit_discarded_ipv6_ipsec_unauthenticated_packets_persec.as_ref()
    }

    /// Sets the value of InboundRateLimitDiscardedDefaultBlockExemptPackets
    pub fn set_inbound_rate_limit_discarded_default_block_exempt_packets(&mut self, value: u64) {
        self.inbound_rate_limit_discarded_default_block_exempt_packets = Some(value);
    }

    /// Gets the value of InboundRateLimitDiscardedDefaultBlockExemptPackets
    pub fn get_inbound_rate_limit_discarded_default_block_exempt_packets(&self) -> Option<&u64> {
        self.inbound_rate_limit_discarded_default_block_exempt_packets.as_ref()
    }

    /// Sets the value of InboundRateLimitDiscardedDefaultBlockExemptPacketsPersec
    pub fn set_inbound_rate_limit_discarded_default_block_exempt_packets_persec(&mut self, value: u32) {
        self.inbound_rate_limit_discarded_default_block_exempt_packets_persec = Some(value);
    }

    /// Gets the value of InboundRateLimitDiscardedDefaultBlockExemptPacketsPersec
    pub fn get_inbound_rate_limit_discarded_default_block_exempt_packets_persec(&self) -> Option<&u32> {
        self.inbound_rate_limit_discarded_default_block_exempt_packets_persec.as_ref()
    }

    /// Sets the value of InboundRateLimitDiscardedFilterExemptIPv6Packets
    pub fn set_inbound_rate_limit_discarded_filter_exempt_ipv6_packets(&mut self, value: u64) {
        self.inbound_rate_limit_discarded_filter_exempt_ipv6_packets = Some(value);
    }

    /// Gets the value of InboundRateLimitDiscardedFilterExemptIPv6Packets
    pub fn get_inbound_rate_limit_discarded_filter_exempt_ipv6_packets(&self) -> Option<&u64> {
        self.inbound_rate_limit_discarded_filter_exempt_ipv6_packets.as_ref()
    }

    /// Sets the value of InboundRateLimitDiscardedFilterExemptIPv6PacketsPersec
    pub fn set_inbound_rate_limit_discarded_filter_exempt_ipv6_packets_persec(&mut self, value: u32) {
        self.inbound_rate_limit_discarded_filter_exempt_ipv6_packets_persec = Some(value);
    }

    /// Gets the value of InboundRateLimitDiscardedFilterExemptIPv6PacketsPersec
    pub fn get_inbound_rate_limit_discarded_filter_exempt_ipv6_packets_persec(&self) -> Option<&u32> {
        self.inbound_rate_limit_discarded_filter_exempt_ipv6_packets_persec.as_ref()
    }

    /// Sets the value of InboundRateLimitDiscardedICMPv6Packets
    pub fn set_inbound_rate_limit_discarded_icmpv6_packets(&mut self, value: u64) {
        self.inbound_rate_limit_discarded_icmpv6_packets = Some(value);
    }

    /// Gets the value of InboundRateLimitDiscardedICMPv6Packets
    pub fn get_inbound_rate_limit_discarded_icmpv6_packets(&self) -> Option<&u64> {
        self.inbound_rate_limit_discarded_icmpv6_packets.as_ref()
    }

    /// Sets the value of InboundRateLimitDiscardedICMPv6PacketsPersec
    pub fn set_inbound_rate_limit_discarded_icmpv6_packets_persec(&mut self, value: u32) {
        self.inbound_rate_limit_discarded_icmpv6_packets_persec = Some(value);
    }

    /// Gets the value of InboundRateLimitDiscardedICMPv6PacketsPersec
    pub fn get_inbound_rate_limit_discarded_icmpv6_packets_persec(&self) -> Option<&u32> {
        self.inbound_rate_limit_discarded_icmpv6_packets_persec.as_ref()
    }

    /// Sets the value of InboundRateLimitDiscardedIPv6IPsecAuthenticatedPackets
    pub fn set_inbound_rate_limit_discarded_ipv6_ipsec_authenticated_packets(&mut self, value: u64) {
        self.inbound_rate_limit_discarded_ipv6_ipsec_authenticated_packets = Some(value);
    }

    /// Gets the value of InboundRateLimitDiscardedIPv6IPsecAuthenticatedPackets
    pub fn get_inbound_rate_limit_discarded_ipv6_ipsec_authenticated_packets(&self) -> Option<&u64> {
        self.inbound_rate_limit_discarded_ipv6_ipsec_authenticated_packets.as_ref()
    }

    /// Sets the value of InboundRateLimitDiscardedIPv6IPsecAuthenticatedPacketsPersec
    pub fn set_inbound_rate_limit_discarded_ipv6_ipsec_authenticated_packets_persec(&mut self, value: u32) {
        self.inbound_rate_limit_discarded_ipv6_ipsec_authenticated_packets_persec = Some(value);
    }

    /// Gets the value of InboundRateLimitDiscardedIPv6IPsecAuthenticatedPacketsPersec
    pub fn get_inbound_rate_limit_discarded_ipv6_ipsec_authenticated_packets_persec(&self) -> Option<&u32> {
        self.inbound_rate_limit_discarded_ipv6_ipsec_authenticated_packets_persec.as_ref()
    }

    /// Sets the value of InboundRateLimitDiscardedIPv6IPsecUnauthenticatedPackets
    pub fn set_inbound_rate_limit_discarded_ipv6_ipsec_unauthenticated_packets(&mut self, value: u64) {
        self.inbound_rate_limit_discarded_ipv6_ipsec_unauthenticated_packets = Some(value);
    }

    /// Gets the value of InboundRateLimitDiscardedIPv6IPsecUnauthenticatedPackets
    pub fn get_inbound_rate_limit_discarded_ipv6_ipsec_unauthenticated_packets(&self) -> Option<&u64> {
        self.inbound_rate_limit_discarded_ipv6_ipsec_unauthenticated_packets.as_ref()
    }

    /// Sets the value of InboundRateLimitDiscardedIPv6IPsecUnauthenticatedPacketsPersec
    pub fn set_inbound_rate_limit_discarded_ipv6_ipsec_unauthenticated_packets_persec(&mut self, value: u32) {
        self.inbound_rate_limit_discarded_ipv6_ipsec_unauthenticated_packets_persec = Some(value);
    }

    /// Gets the value of InboundRateLimitDiscardedIPv6IPsecUnauthenticatedPacketsPersec
    pub fn get_inbound_rate_limit_discarded_ipv6_ipsec_unauthenticated_packets_persec(&self) -> Option<&u32> {
        self.inbound_rate_limit_discarded_ipv6_ipsec_unauthenticated_packets_persec.as_ref()
    }

    /// Sets the value of PerIPRateLimitQueues
    pub fn set_per_iprate_limit_queues(&mut self, value: u64) {
        self.per_iprate_limit_queues = Some(value);
    }

    /// Gets the value of PerIPRateLimitQueues
    pub fn get_per_iprate_limit_queues(&self) -> Option<&u64> {
        self.per_iprate_limit_queues.as_ref()
    }

    /// Sets the value of StateEntries
    pub fn set_state_entries(&mut self, value: u64) {
        self.state_entries = Some(value);
    }

    /// Gets the value of StateEntries
    pub fn get_state_entries(&self) -> Option<&u64> {
        self.state_entries.as_ref()
    }

    /// Sets the value of StateEntriesPersec
    pub fn set_state_entries_persec(&mut self, value: u32) {
        self.state_entries_persec = Some(value);
    }

    /// Gets the value of StateEntriesPersec
    pub fn get_state_entries_persec(&self) -> Option<&u32> {
        self.state_entries_persec.as_ref()
    }
}

