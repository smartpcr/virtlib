// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetIPInterface struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetIPInterface {
    #[serde(flatten)]
    pub base: CIM_LANEndpoint,

/// 
    #[serde(rename = "AddressFamily")]
    pub address_family: Option<u16>,

/// 
    #[serde(rename = "AdvertiseDefaultRoute")]
    pub advertise_default_route: Option<u8>,

/// 
    #[serde(rename = "AdvertisedRouterLifetime")]
    pub advertised_router_lifetime: Option<String>,

/// 
    #[serde(rename = "Advertising")]
    pub advertising: Option<u8>,

/// 
    #[serde(rename = "AutomaticMetric")]
    pub automatic_metric: Option<u8>,

/// 
    #[serde(rename = "BaseReachableTime")]
    pub base_reachable_time: Option<u32>,

/// 
    #[serde(rename = "ClampMss")]
    pub clamp_mss: Option<u8>,

/// 
    #[serde(rename = "CompartmentId")]
    pub compartment_id: Option<u32>,

/// 
    #[serde(rename = "ConnectionState")]
    pub connection_state: Option<u8>,

/// 
    #[serde(rename = "CurrentHopLimit")]
    pub current_hop_limit: Option<u32>,

/// 
    #[serde(rename = "DadRetransmitTime")]
    pub dad_retransmit_time: Option<u32>,

/// 
    #[serde(rename = "DadTransmits")]
    pub dad_transmits: Option<u32>,

/// 
    #[serde(rename = "Dhcp")]
    pub dhcp: Option<u8>,

/// 
    #[serde(rename = "DirectedMacWolPattern")]
    pub directed_mac_wol_pattern: Option<u8>,

/// 
    #[serde(rename = "EcnMarking")]
    pub ecn_marking: Option<u8>,

/// 
    #[serde(rename = "ForceArpNdWolPattern")]
    pub force_arp_nd_wol_pattern: Option<u8>,

/// 
    #[serde(rename = "Forwarding")]
    pub forwarding: Option<u8>,

/// 
    #[serde(rename = "IgnoreDefaultRoutes")]
    pub ignore_default_routes: Option<u8>,

/// 
    #[serde(rename = "InterfaceAlias")]
    pub interface_alias: Option<String>,

/// 
    #[serde(rename = "InterfaceIndex")]
    pub interface_index: Option<u32>,

/// 
    #[serde(rename = "InterfaceMetric")]
    pub interface_metric: Option<u32>,

/// 
    #[serde(rename = "IsolationId")]
    pub isolation_id: Option<u32>,

/// 
    #[serde(rename = "LowestIfNetLuid")]
    pub lowest_if_net_luid: Option<u64>,

/// 
    #[serde(rename = "ManagedAddressConfiguration")]
    pub managed_address_configuration: Option<u8>,

/// 
    #[serde(rename = "NeighborDiscoverySupported")]
    pub neighbor_discovery_supported: Option<u8>,

/// 
    #[serde(rename = "NeighborUnreachabilityDetection")]
    pub neighbor_unreachability_detection: Option<u8>,

/// 
    #[serde(rename = "NlMtu")]
    pub nl_mtu: Option<u32>,

/// 
    #[serde(rename = "OtherStatefulConfiguration")]
    pub other_stateful_configuration: Option<u8>,

/// 
    #[serde(rename = "ReachableTime")]
    pub reachable_time: Option<u32>,

/// 
    #[serde(rename = "RetransmitTime")]
    pub retransmit_time: Option<u32>,

/// 
    #[serde(rename = "RouterDiscovery")]
    pub router_discovery: Option<u8>,

/// 
    #[serde(rename = "Store")]
    pub store: Option<u8>,

/// 
    #[serde(rename = "WeakHostReceive")]
    pub weak_host_receive: Option<u8>,

/// 
    #[serde(rename = "WeakHostSend")]
    pub weak_host_send: Option<u8>,
}

impl MSFT_NetIPInterface {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LANEndpoint::new(),
            address_family: None,
            advertise_default_route: None,
            advertised_router_lifetime: None,
            advertising: None,
            automatic_metric: None,
            base_reachable_time: None,
            clamp_mss: None,
            compartment_id: None,
            connection_state: None,
            current_hop_limit: None,
            dad_retransmit_time: None,
            dad_transmits: None,
            dhcp: None,
            directed_mac_wol_pattern: None,
            ecn_marking: None,
            force_arp_nd_wol_pattern: None,
            forwarding: None,
            ignore_default_routes: None,
            interface_alias: None,
            interface_index: None,
            interface_metric: None,
            isolation_id: None,
            lowest_if_net_luid: None,
            managed_address_configuration: None,
            neighbor_discovery_supported: None,
            neighbor_unreachability_detection: None,
            nl_mtu: None,
            other_stateful_configuration: None,
            reachable_time: None,
            retransmit_time: None,
            router_discovery: None,
            store: None,
            weak_host_receive: None,
            weak_host_send: None,
        }
    }


    /// Sets the value of AddressFamily
    pub fn set_address_family(&mut self, value: u16) {
        self.address_family = Some(value);
    }

    /// Gets the value of AddressFamily
    pub fn get_address_family(&self) -> Option<&u16> {
        self.address_family.as_ref()
    }

    /// Sets the value of AdvertiseDefaultRoute
    pub fn set_advertise_default_route(&mut self, value: u8) {
        self.advertise_default_route = Some(value);
    }

    /// Gets the value of AdvertiseDefaultRoute
    pub fn get_advertise_default_route(&self) -> Option<&u8> {
        self.advertise_default_route.as_ref()
    }

    /// Sets the value of AdvertisedRouterLifetime
    pub fn set_advertised_router_lifetime(&mut self, value: String) {
        self.advertised_router_lifetime = Some(value);
    }

    /// Gets the value of AdvertisedRouterLifetime
    pub fn get_advertised_router_lifetime(&self) -> Option<&String> {
        self.advertised_router_lifetime.as_ref()
    }

    /// Sets the value of Advertising
    pub fn set_advertising(&mut self, value: u8) {
        self.advertising = Some(value);
    }

    /// Gets the value of Advertising
    pub fn get_advertising(&self) -> Option<&u8> {
        self.advertising.as_ref()
    }

    /// Sets the value of AutomaticMetric
    pub fn set_automatic_metric(&mut self, value: u8) {
        self.automatic_metric = Some(value);
    }

    /// Gets the value of AutomaticMetric
    pub fn get_automatic_metric(&self) -> Option<&u8> {
        self.automatic_metric.as_ref()
    }

    /// Sets the value of BaseReachableTime
    pub fn set_base_reachable_time(&mut self, value: u32) {
        self.base_reachable_time = Some(value);
    }

    /// Gets the value of BaseReachableTime
    pub fn get_base_reachable_time(&self) -> Option<&u32> {
        self.base_reachable_time.as_ref()
    }

    /// Sets the value of ClampMss
    pub fn set_clamp_mss(&mut self, value: u8) {
        self.clamp_mss = Some(value);
    }

    /// Gets the value of ClampMss
    pub fn get_clamp_mss(&self) -> Option<&u8> {
        self.clamp_mss.as_ref()
    }

    /// Sets the value of CompartmentId
    pub fn set_compartment_id(&mut self, value: u32) {
        self.compartment_id = Some(value);
    }

    /// Gets the value of CompartmentId
    pub fn get_compartment_id(&self) -> Option<&u32> {
        self.compartment_id.as_ref()
    }

    /// Sets the value of ConnectionState
    pub fn set_connection_state(&mut self, value: u8) {
        self.connection_state = Some(value);
    }

    /// Gets the value of ConnectionState
    pub fn get_connection_state(&self) -> Option<&u8> {
        self.connection_state.as_ref()
    }

    /// Sets the value of CurrentHopLimit
    pub fn set_current_hop_limit(&mut self, value: u32) {
        self.current_hop_limit = Some(value);
    }

    /// Gets the value of CurrentHopLimit
    pub fn get_current_hop_limit(&self) -> Option<&u32> {
        self.current_hop_limit.as_ref()
    }

    /// Sets the value of DadRetransmitTime
    pub fn set_dad_retransmit_time(&mut self, value: u32) {
        self.dad_retransmit_time = Some(value);
    }

    /// Gets the value of DadRetransmitTime
    pub fn get_dad_retransmit_time(&self) -> Option<&u32> {
        self.dad_retransmit_time.as_ref()
    }

    /// Sets the value of DadTransmits
    pub fn set_dad_transmits(&mut self, value: u32) {
        self.dad_transmits = Some(value);
    }

    /// Gets the value of DadTransmits
    pub fn get_dad_transmits(&self) -> Option<&u32> {
        self.dad_transmits.as_ref()
    }

    /// Sets the value of Dhcp
    pub fn set_dhcp(&mut self, value: u8) {
        self.dhcp = Some(value);
    }

    /// Gets the value of Dhcp
    pub fn get_dhcp(&self) -> Option<&u8> {
        self.dhcp.as_ref()
    }

    /// Sets the value of DirectedMacWolPattern
    pub fn set_directed_mac_wol_pattern(&mut self, value: u8) {
        self.directed_mac_wol_pattern = Some(value);
    }

    /// Gets the value of DirectedMacWolPattern
    pub fn get_directed_mac_wol_pattern(&self) -> Option<&u8> {
        self.directed_mac_wol_pattern.as_ref()
    }

    /// Sets the value of EcnMarking
    pub fn set_ecn_marking(&mut self, value: u8) {
        self.ecn_marking = Some(value);
    }

    /// Gets the value of EcnMarking
    pub fn get_ecn_marking(&self) -> Option<&u8> {
        self.ecn_marking.as_ref()
    }

    /// Sets the value of ForceArpNdWolPattern
    pub fn set_force_arp_nd_wol_pattern(&mut self, value: u8) {
        self.force_arp_nd_wol_pattern = Some(value);
    }

    /// Gets the value of ForceArpNdWolPattern
    pub fn get_force_arp_nd_wol_pattern(&self) -> Option<&u8> {
        self.force_arp_nd_wol_pattern.as_ref()
    }

    /// Sets the value of Forwarding
    pub fn set_forwarding(&mut self, value: u8) {
        self.forwarding = Some(value);
    }

    /// Gets the value of Forwarding
    pub fn get_forwarding(&self) -> Option<&u8> {
        self.forwarding.as_ref()
    }

    /// Sets the value of IgnoreDefaultRoutes
    pub fn set_ignore_default_routes(&mut self, value: u8) {
        self.ignore_default_routes = Some(value);
    }

    /// Gets the value of IgnoreDefaultRoutes
    pub fn get_ignore_default_routes(&self) -> Option<&u8> {
        self.ignore_default_routes.as_ref()
    }

    /// Sets the value of InterfaceAlias
    pub fn set_interface_alias(&mut self, value: String) {
        self.interface_alias = Some(value);
    }

    /// Gets the value of InterfaceAlias
    pub fn get_interface_alias(&self) -> Option<&String> {
        self.interface_alias.as_ref()
    }

    /// Sets the value of InterfaceIndex
    pub fn set_interface_index(&mut self, value: u32) {
        self.interface_index = Some(value);
    }

    /// Gets the value of InterfaceIndex
    pub fn get_interface_index(&self) -> Option<&u32> {
        self.interface_index.as_ref()
    }

    /// Sets the value of InterfaceMetric
    pub fn set_interface_metric(&mut self, value: u32) {
        self.interface_metric = Some(value);
    }

    /// Gets the value of InterfaceMetric
    pub fn get_interface_metric(&self) -> Option<&u32> {
        self.interface_metric.as_ref()
    }

    /// Sets the value of IsolationId
    pub fn set_isolation_id(&mut self, value: u32) {
        self.isolation_id = Some(value);
    }

    /// Gets the value of IsolationId
    pub fn get_isolation_id(&self) -> Option<&u32> {
        self.isolation_id.as_ref()
    }

    /// Sets the value of LowestIfNetLuid
    pub fn set_lowest_if_net_luid(&mut self, value: u64) {
        self.lowest_if_net_luid = Some(value);
    }

    /// Gets the value of LowestIfNetLuid
    pub fn get_lowest_if_net_luid(&self) -> Option<&u64> {
        self.lowest_if_net_luid.as_ref()
    }

    /// Sets the value of ManagedAddressConfiguration
    pub fn set_managed_address_configuration(&mut self, value: u8) {
        self.managed_address_configuration = Some(value);
    }

    /// Gets the value of ManagedAddressConfiguration
    pub fn get_managed_address_configuration(&self) -> Option<&u8> {
        self.managed_address_configuration.as_ref()
    }

    /// Sets the value of NeighborDiscoverySupported
    pub fn set_neighbor_discovery_supported(&mut self, value: u8) {
        self.neighbor_discovery_supported = Some(value);
    }

    /// Gets the value of NeighborDiscoverySupported
    pub fn get_neighbor_discovery_supported(&self) -> Option<&u8> {
        self.neighbor_discovery_supported.as_ref()
    }

    /// Sets the value of NeighborUnreachabilityDetection
    pub fn set_neighbor_unreachability_detection(&mut self, value: u8) {
        self.neighbor_unreachability_detection = Some(value);
    }

    /// Gets the value of NeighborUnreachabilityDetection
    pub fn get_neighbor_unreachability_detection(&self) -> Option<&u8> {
        self.neighbor_unreachability_detection.as_ref()
    }

    /// Sets the value of NlMtu
    pub fn set_nl_mtu(&mut self, value: u32) {
        self.nl_mtu = Some(value);
    }

    /// Gets the value of NlMtu
    pub fn get_nl_mtu(&self) -> Option<&u32> {
        self.nl_mtu.as_ref()
    }

    /// Sets the value of OtherStatefulConfiguration
    pub fn set_other_stateful_configuration(&mut self, value: u8) {
        self.other_stateful_configuration = Some(value);
    }

    /// Gets the value of OtherStatefulConfiguration
    pub fn get_other_stateful_configuration(&self) -> Option<&u8> {
        self.other_stateful_configuration.as_ref()
    }

    /// Sets the value of ReachableTime
    pub fn set_reachable_time(&mut self, value: u32) {
        self.reachable_time = Some(value);
    }

    /// Gets the value of ReachableTime
    pub fn get_reachable_time(&self) -> Option<&u32> {
        self.reachable_time.as_ref()
    }

    /// Sets the value of RetransmitTime
    pub fn set_retransmit_time(&mut self, value: u32) {
        self.retransmit_time = Some(value);
    }

    /// Gets the value of RetransmitTime
    pub fn get_retransmit_time(&self) -> Option<&u32> {
        self.retransmit_time.as_ref()
    }

    /// Sets the value of RouterDiscovery
    pub fn set_router_discovery(&mut self, value: u8) {
        self.router_discovery = Some(value);
    }

    /// Gets the value of RouterDiscovery
    pub fn get_router_discovery(&self) -> Option<&u8> {
        self.router_discovery.as_ref()
    }

    /// Sets the value of Store
    pub fn set_store(&mut self, value: u8) {
        self.store = Some(value);
    }

    /// Gets the value of Store
    pub fn get_store(&self) -> Option<&u8> {
        self.store.as_ref()
    }

    /// Sets the value of WeakHostReceive
    pub fn set_weak_host_receive(&mut self, value: u8) {
        self.weak_host_receive = Some(value);
    }

    /// Gets the value of WeakHostReceive
    pub fn get_weak_host_receive(&self) -> Option<&u8> {
        self.weak_host_receive.as_ref()
    }

    /// Sets the value of WeakHostSend
    pub fn set_weak_host_send(&mut self, value: u8) {
        self.weak_host_send = Some(value);
    }

    /// Gets the value of WeakHostSend
    pub fn get_weak_host_send(&self) -> Option<&u8> {
        self.weak_host_send.as_ref()
    }
}

