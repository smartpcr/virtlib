// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_WinNatCounters_WinNAT struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_WinNatCounters_WinNAT {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "CurrentSessionCount")]
    pub current_session_count: Option<u32>,

/// 
    #[serde(rename = "DroppedICMPerrorpackets")]
    pub dropped_icmperrorpackets: Option<u32>,

/// 
    #[serde(rename = "DroppedICMPerrorpacketsPersec")]
    pub dropped_icmperrorpackets_persec: Option<u32>,

/// 
    #[serde(rename = "DroppedPackets")]
    pub dropped_packets: Option<u32>,

/// 
    #[serde(rename = "DroppedPacketsPersec")]
    pub dropped_packets_persec: Option<u32>,

/// 
    #[serde(rename = "InterRoutingDomainHairpinnedPackets")]
    pub inter_routing_domain_hairpinned_packets: Option<u32>,

/// 
    #[serde(rename = "InterRoutingDomainHairpinnedPacketsPersec")]
    pub inter_routing_domain_hairpinned_packets_persec: Option<u32>,

/// 
    #[serde(rename = "IntraRoutingDomainHairpinnedPackets")]
    pub intra_routing_domain_hairpinned_packets: Option<u32>,

/// 
    #[serde(rename = "IntraRoutingDomainHairpinnedPacketsPersec")]
    pub intra_routing_domain_hairpinned_packets_persec: Option<u32>,

/// 
    #[serde(rename = "PacketsExternaltoInternal")]
    pub packets_externalto_internal: Option<u32>,

/// 
    #[serde(rename = "PacketsInternaltoExternal")]
    pub packets_internalto_external: Option<u32>,

/// 
    #[serde(rename = "PacketsPersecExternaltoInternal")]
    pub packets_persec_externalto_internal: Option<u32>,

/// 
    #[serde(rename = "PacketsPersecInternaltoExternal")]
    pub packets_persec_internalto_external: Option<u32>,

/// 
    #[serde(rename = "SessionsPersec")]
    pub sessions_persec: Option<u32>,
}

impl Win32_PerfFormattedData_WinNatCounters_WinNAT {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            current_session_count: None,
            dropped_icmperrorpackets: None,
            dropped_icmperrorpackets_persec: None,
            dropped_packets: None,
            dropped_packets_persec: None,
            inter_routing_domain_hairpinned_packets: None,
            inter_routing_domain_hairpinned_packets_persec: None,
            intra_routing_domain_hairpinned_packets: None,
            intra_routing_domain_hairpinned_packets_persec: None,
            packets_externalto_internal: None,
            packets_internalto_external: None,
            packets_persec_externalto_internal: None,
            packets_persec_internalto_external: None,
            sessions_persec: None,
        }
    }


    /// Sets the value of CurrentSessionCount
    pub fn set_current_session_count(&mut self, value: u32) {
        self.current_session_count = Some(value);
    }

    /// Gets the value of CurrentSessionCount
    pub fn get_current_session_count(&self) -> Option<&u32> {
        self.current_session_count.as_ref()
    }

    /// Sets the value of DroppedICMPerrorpackets
    pub fn set_dropped_icmperrorpackets(&mut self, value: u32) {
        self.dropped_icmperrorpackets = Some(value);
    }

    /// Gets the value of DroppedICMPerrorpackets
    pub fn get_dropped_icmperrorpackets(&self) -> Option<&u32> {
        self.dropped_icmperrorpackets.as_ref()
    }

    /// Sets the value of DroppedICMPerrorpacketsPersec
    pub fn set_dropped_icmperrorpackets_persec(&mut self, value: u32) {
        self.dropped_icmperrorpackets_persec = Some(value);
    }

    /// Gets the value of DroppedICMPerrorpacketsPersec
    pub fn get_dropped_icmperrorpackets_persec(&self) -> Option<&u32> {
        self.dropped_icmperrorpackets_persec.as_ref()
    }

    /// Sets the value of DroppedPackets
    pub fn set_dropped_packets(&mut self, value: u32) {
        self.dropped_packets = Some(value);
    }

    /// Gets the value of DroppedPackets
    pub fn get_dropped_packets(&self) -> Option<&u32> {
        self.dropped_packets.as_ref()
    }

    /// Sets the value of DroppedPacketsPersec
    pub fn set_dropped_packets_persec(&mut self, value: u32) {
        self.dropped_packets_persec = Some(value);
    }

    /// Gets the value of DroppedPacketsPersec
    pub fn get_dropped_packets_persec(&self) -> Option<&u32> {
        self.dropped_packets_persec.as_ref()
    }

    /// Sets the value of InterRoutingDomainHairpinnedPackets
    pub fn set_inter_routing_domain_hairpinned_packets(&mut self, value: u32) {
        self.inter_routing_domain_hairpinned_packets = Some(value);
    }

    /// Gets the value of InterRoutingDomainHairpinnedPackets
    pub fn get_inter_routing_domain_hairpinned_packets(&self) -> Option<&u32> {
        self.inter_routing_domain_hairpinned_packets.as_ref()
    }

    /// Sets the value of InterRoutingDomainHairpinnedPacketsPersec
    pub fn set_inter_routing_domain_hairpinned_packets_persec(&mut self, value: u32) {
        self.inter_routing_domain_hairpinned_packets_persec = Some(value);
    }

    /// Gets the value of InterRoutingDomainHairpinnedPacketsPersec
    pub fn get_inter_routing_domain_hairpinned_packets_persec(&self) -> Option<&u32> {
        self.inter_routing_domain_hairpinned_packets_persec.as_ref()
    }

    /// Sets the value of IntraRoutingDomainHairpinnedPackets
    pub fn set_intra_routing_domain_hairpinned_packets(&mut self, value: u32) {
        self.intra_routing_domain_hairpinned_packets = Some(value);
    }

    /// Gets the value of IntraRoutingDomainHairpinnedPackets
    pub fn get_intra_routing_domain_hairpinned_packets(&self) -> Option<&u32> {
        self.intra_routing_domain_hairpinned_packets.as_ref()
    }

    /// Sets the value of IntraRoutingDomainHairpinnedPacketsPersec
    pub fn set_intra_routing_domain_hairpinned_packets_persec(&mut self, value: u32) {
        self.intra_routing_domain_hairpinned_packets_persec = Some(value);
    }

    /// Gets the value of IntraRoutingDomainHairpinnedPacketsPersec
    pub fn get_intra_routing_domain_hairpinned_packets_persec(&self) -> Option<&u32> {
        self.intra_routing_domain_hairpinned_packets_persec.as_ref()
    }

    /// Sets the value of PacketsExternaltoInternal
    pub fn set_packets_externalto_internal(&mut self, value: u32) {
        self.packets_externalto_internal = Some(value);
    }

    /// Gets the value of PacketsExternaltoInternal
    pub fn get_packets_externalto_internal(&self) -> Option<&u32> {
        self.packets_externalto_internal.as_ref()
    }

    /// Sets the value of PacketsInternaltoExternal
    pub fn set_packets_internalto_external(&mut self, value: u32) {
        self.packets_internalto_external = Some(value);
    }

    /// Gets the value of PacketsInternaltoExternal
    pub fn get_packets_internalto_external(&self) -> Option<&u32> {
        self.packets_internalto_external.as_ref()
    }

    /// Sets the value of PacketsPersecExternaltoInternal
    pub fn set_packets_persec_externalto_internal(&mut self, value: u32) {
        self.packets_persec_externalto_internal = Some(value);
    }

    /// Gets the value of PacketsPersecExternaltoInternal
    pub fn get_packets_persec_externalto_internal(&self) -> Option<&u32> {
        self.packets_persec_externalto_internal.as_ref()
    }

    /// Sets the value of PacketsPersecInternaltoExternal
    pub fn set_packets_persec_internalto_external(&mut self, value: u32) {
        self.packets_persec_internalto_external = Some(value);
    }

    /// Gets the value of PacketsPersecInternaltoExternal
    pub fn get_packets_persec_internalto_external(&self) -> Option<&u32> {
        self.packets_persec_internalto_external.as_ref()
    }

    /// Sets the value of SessionsPersec
    pub fn set_sessions_persec(&mut self, value: u32) {
        self.sessions_persec = Some(value);
    }

    /// Gets the value of SessionsPersec
    pub fn get_sessions_persec(&self) -> Option<&u32> {
        self.sessions_persec.as_ref()
    }
}

