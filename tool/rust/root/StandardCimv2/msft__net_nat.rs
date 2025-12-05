// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetNat struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetNat {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "Active")]
    pub active: Option<u8>,

/// 
    #[serde(rename = "ExternalIPInterfaceAddressPrefix")]
    pub external_ipinterface_address_prefix: Option<String>,

/// 
    #[serde(rename = "IcmpQueryTimeout")]
    pub icmp_query_timeout: Option<u32>,

/// 
    #[serde(rename = "InternalIPInterfaceAddressPrefix")]
    pub internal_ipinterface_address_prefix: Option<String>,

/// 
    #[serde(rename = "InternalRoutingDomainId")]
    pub internal_routing_domain_id: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Store")]
    pub store: Option<u32>,

/// 
    #[serde(rename = "TcpEstablishedConnectionTimeout")]
    pub tcp_established_connection_timeout: Option<u32>,

/// 
    #[serde(rename = "TcpFilteringBehavior")]
    pub tcp_filtering_behavior: Option<u8>,

/// 
    #[serde(rename = "TcpTransientConnectionTimeout")]
    pub tcp_transient_connection_timeout: Option<u32>,

/// 
    #[serde(rename = "UdpFilteringBehavior")]
    pub udp_filtering_behavior: Option<u8>,

/// 
    #[serde(rename = "UdpIdleSessionTimeout")]
    pub udp_idle_session_timeout: Option<u32>,

/// 
    #[serde(rename = "UdpInboundRefresh")]
    pub udp_inbound_refresh: Option<u8>,
}

impl MSFT_NetNat {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            active: None,
            external_ipinterface_address_prefix: None,
            icmp_query_timeout: None,
            internal_ipinterface_address_prefix: None,
            internal_routing_domain_id: None,
            name: None,
            store: None,
            tcp_established_connection_timeout: None,
            tcp_filtering_behavior: None,
            tcp_transient_connection_timeout: None,
            udp_filtering_behavior: None,
            udp_idle_session_timeout: None,
            udp_inbound_refresh: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: u8) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&u8> {
        self.active.as_ref()
    }

    /// Sets the value of ExternalIPInterfaceAddressPrefix
    pub fn set_external_ipinterface_address_prefix(&mut self, value: String) {
        self.external_ipinterface_address_prefix = Some(value);
    }

    /// Gets the value of ExternalIPInterfaceAddressPrefix
    pub fn get_external_ipinterface_address_prefix(&self) -> Option<&String> {
        self.external_ipinterface_address_prefix.as_ref()
    }

    /// Sets the value of IcmpQueryTimeout
    pub fn set_icmp_query_timeout(&mut self, value: u32) {
        self.icmp_query_timeout = Some(value);
    }

    /// Gets the value of IcmpQueryTimeout
    pub fn get_icmp_query_timeout(&self) -> Option<&u32> {
        self.icmp_query_timeout.as_ref()
    }

    /// Sets the value of InternalIPInterfaceAddressPrefix
    pub fn set_internal_ipinterface_address_prefix(&mut self, value: String) {
        self.internal_ipinterface_address_prefix = Some(value);
    }

    /// Gets the value of InternalIPInterfaceAddressPrefix
    pub fn get_internal_ipinterface_address_prefix(&self) -> Option<&String> {
        self.internal_ipinterface_address_prefix.as_ref()
    }

    /// Sets the value of InternalRoutingDomainId
    pub fn set_internal_routing_domain_id(&mut self, value: String) {
        self.internal_routing_domain_id = Some(value);
    }

    /// Gets the value of InternalRoutingDomainId
    pub fn get_internal_routing_domain_id(&self) -> Option<&String> {
        self.internal_routing_domain_id.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Store
    pub fn set_store(&mut self, value: u32) {
        self.store = Some(value);
    }

    /// Gets the value of Store
    pub fn get_store(&self) -> Option<&u32> {
        self.store.as_ref()
    }

    /// Sets the value of TcpEstablishedConnectionTimeout
    pub fn set_tcp_established_connection_timeout(&mut self, value: u32) {
        self.tcp_established_connection_timeout = Some(value);
    }

    /// Gets the value of TcpEstablishedConnectionTimeout
    pub fn get_tcp_established_connection_timeout(&self) -> Option<&u32> {
        self.tcp_established_connection_timeout.as_ref()
    }

    /// Sets the value of TcpFilteringBehavior
    pub fn set_tcp_filtering_behavior(&mut self, value: u8) {
        self.tcp_filtering_behavior = Some(value);
    }

    /// Gets the value of TcpFilteringBehavior
    pub fn get_tcp_filtering_behavior(&self) -> Option<&u8> {
        self.tcp_filtering_behavior.as_ref()
    }

    /// Sets the value of TcpTransientConnectionTimeout
    pub fn set_tcp_transient_connection_timeout(&mut self, value: u32) {
        self.tcp_transient_connection_timeout = Some(value);
    }

    /// Gets the value of TcpTransientConnectionTimeout
    pub fn get_tcp_transient_connection_timeout(&self) -> Option<&u32> {
        self.tcp_transient_connection_timeout.as_ref()
    }

    /// Sets the value of UdpFilteringBehavior
    pub fn set_udp_filtering_behavior(&mut self, value: u8) {
        self.udp_filtering_behavior = Some(value);
    }

    /// Gets the value of UdpFilteringBehavior
    pub fn get_udp_filtering_behavior(&self) -> Option<&u8> {
        self.udp_filtering_behavior.as_ref()
    }

    /// Sets the value of UdpIdleSessionTimeout
    pub fn set_udp_idle_session_timeout(&mut self, value: u32) {
        self.udp_idle_session_timeout = Some(value);
    }

    /// Gets the value of UdpIdleSessionTimeout
    pub fn get_udp_idle_session_timeout(&self) -> Option<&u32> {
        self.udp_idle_session_timeout.as_ref()
    }

    /// Sets the value of UdpInboundRefresh
    pub fn set_udp_inbound_refresh(&mut self, value: u8) {
        self.udp_inbound_refresh = Some(value);
    }

    /// Gets the value of UdpInboundRefresh
    pub fn get_udp_inbound_refresh(&self) -> Option<&u8> {
        self.udp_inbound_refresh.as_ref()
    }
}

