// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetBaseIPProtocol struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetBaseIPProtocol {
    #[serde(flatten)]
    pub base: CIM_ProtocolEndpoint,

/// 
    #[serde(rename = "AddressMaskReply")]
    pub address_mask_reply: Option<u8>,

/// 
    #[serde(rename = "DeadGatewayDetection")]
    pub dead_gateway_detection: Option<u8>,

/// 
    #[serde(rename = "DefaultHopLimit")]
    pub default_hop_limit: Option<u32>,

/// 
    #[serde(rename = "DhcpMediaSense")]
    pub dhcp_media_sense: Option<u8>,

/// 
    #[serde(rename = "GroupForwardedFragments")]
    pub group_forwarded_fragments: Option<u8>,

/// 
    #[serde(rename = "IcmpRedirects")]
    pub icmp_redirects: Option<u8>,

/// 
    #[serde(rename = "MediaSenseEventLog")]
    pub media_sense_event_log: Option<u8>,

/// 
    #[serde(rename = "MldLevel")]
    pub mld_level: Option<u32>,

/// 
    #[serde(rename = "MldVersion")]
    pub mld_version: Option<u32>,

/// 
    #[serde(rename = "MulticastForwarding")]
    pub multicast_forwarding: Option<u8>,

/// 
    #[serde(rename = "NeighborCacheLimit")]
    pub neighbor_cache_limit: Option<u32>,

/// 
    #[serde(rename = "RandomizeIdentifiers")]
    pub randomize_identifiers: Option<u8>,

/// 
    #[serde(rename = "ReassemblyLimit")]
    pub reassembly_limit: Option<u32>,

/// 
    #[serde(rename = "RouteCacheLimit")]
    pub route_cache_limit: Option<u32>,

/// 
    #[serde(rename = "SourceRoutingBehavior")]
    pub source_routing_behavior: Option<u32>,
}

impl MSFT_NetBaseIPProtocol {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ProtocolEndpoint::new(),
            address_mask_reply: None,
            dead_gateway_detection: None,
            default_hop_limit: None,
            dhcp_media_sense: None,
            group_forwarded_fragments: None,
            icmp_redirects: None,
            media_sense_event_log: None,
            mld_level: None,
            mld_version: None,
            multicast_forwarding: None,
            neighbor_cache_limit: None,
            randomize_identifiers: None,
            reassembly_limit: None,
            route_cache_limit: None,
            source_routing_behavior: None,
        }
    }


    /// Sets the value of AddressMaskReply
    pub fn set_address_mask_reply(&mut self, value: u8) {
        self.address_mask_reply = Some(value);
    }

    /// Gets the value of AddressMaskReply
    pub fn get_address_mask_reply(&self) -> Option<&u8> {
        self.address_mask_reply.as_ref()
    }

    /// Sets the value of DeadGatewayDetection
    pub fn set_dead_gateway_detection(&mut self, value: u8) {
        self.dead_gateway_detection = Some(value);
    }

    /// Gets the value of DeadGatewayDetection
    pub fn get_dead_gateway_detection(&self) -> Option<&u8> {
        self.dead_gateway_detection.as_ref()
    }

    /// Sets the value of DefaultHopLimit
    pub fn set_default_hop_limit(&mut self, value: u32) {
        self.default_hop_limit = Some(value);
    }

    /// Gets the value of DefaultHopLimit
    pub fn get_default_hop_limit(&self) -> Option<&u32> {
        self.default_hop_limit.as_ref()
    }

    /// Sets the value of DhcpMediaSense
    pub fn set_dhcp_media_sense(&mut self, value: u8) {
        self.dhcp_media_sense = Some(value);
    }

    /// Gets the value of DhcpMediaSense
    pub fn get_dhcp_media_sense(&self) -> Option<&u8> {
        self.dhcp_media_sense.as_ref()
    }

    /// Sets the value of GroupForwardedFragments
    pub fn set_group_forwarded_fragments(&mut self, value: u8) {
        self.group_forwarded_fragments = Some(value);
    }

    /// Gets the value of GroupForwardedFragments
    pub fn get_group_forwarded_fragments(&self) -> Option<&u8> {
        self.group_forwarded_fragments.as_ref()
    }

    /// Sets the value of IcmpRedirects
    pub fn set_icmp_redirects(&mut self, value: u8) {
        self.icmp_redirects = Some(value);
    }

    /// Gets the value of IcmpRedirects
    pub fn get_icmp_redirects(&self) -> Option<&u8> {
        self.icmp_redirects.as_ref()
    }

    /// Sets the value of MediaSenseEventLog
    pub fn set_media_sense_event_log(&mut self, value: u8) {
        self.media_sense_event_log = Some(value);
    }

    /// Gets the value of MediaSenseEventLog
    pub fn get_media_sense_event_log(&self) -> Option<&u8> {
        self.media_sense_event_log.as_ref()
    }

    /// Sets the value of MldLevel
    pub fn set_mld_level(&mut self, value: u32) {
        self.mld_level = Some(value);
    }

    /// Gets the value of MldLevel
    pub fn get_mld_level(&self) -> Option<&u32> {
        self.mld_level.as_ref()
    }

    /// Sets the value of MldVersion
    pub fn set_mld_version(&mut self, value: u32) {
        self.mld_version = Some(value);
    }

    /// Gets the value of MldVersion
    pub fn get_mld_version(&self) -> Option<&u32> {
        self.mld_version.as_ref()
    }

    /// Sets the value of MulticastForwarding
    pub fn set_multicast_forwarding(&mut self, value: u8) {
        self.multicast_forwarding = Some(value);
    }

    /// Gets the value of MulticastForwarding
    pub fn get_multicast_forwarding(&self) -> Option<&u8> {
        self.multicast_forwarding.as_ref()
    }

    /// Sets the value of NeighborCacheLimit
    pub fn set_neighbor_cache_limit(&mut self, value: u32) {
        self.neighbor_cache_limit = Some(value);
    }

    /// Gets the value of NeighborCacheLimit
    pub fn get_neighbor_cache_limit(&self) -> Option<&u32> {
        self.neighbor_cache_limit.as_ref()
    }

    /// Sets the value of RandomizeIdentifiers
    pub fn set_randomize_identifiers(&mut self, value: u8) {
        self.randomize_identifiers = Some(value);
    }

    /// Gets the value of RandomizeIdentifiers
    pub fn get_randomize_identifiers(&self) -> Option<&u8> {
        self.randomize_identifiers.as_ref()
    }

    /// Sets the value of ReassemblyLimit
    pub fn set_reassembly_limit(&mut self, value: u32) {
        self.reassembly_limit = Some(value);
    }

    /// Gets the value of ReassemblyLimit
    pub fn get_reassembly_limit(&self) -> Option<&u32> {
        self.reassembly_limit.as_ref()
    }

    /// Sets the value of RouteCacheLimit
    pub fn set_route_cache_limit(&mut self, value: u32) {
        self.route_cache_limit = Some(value);
    }

    /// Gets the value of RouteCacheLimit
    pub fn get_route_cache_limit(&self) -> Option<&u32> {
        self.route_cache_limit.as_ref()
    }

    /// Sets the value of SourceRoutingBehavior
    pub fn set_source_routing_behavior(&mut self, value: u32) {
        self.source_routing_behavior = Some(value);
    }

    /// Gets the value of SourceRoutingBehavior
    pub fn get_source_routing_behavior(&self) -> Option<&u32> {
        self.source_routing_behavior.as_ref()
    }
}

