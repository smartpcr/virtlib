// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetNatStaticMapping struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetNatStaticMapping {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "Active")]
    pub active: Option<u8>,

/// 
    #[serde(rename = "ExternalIPAddress")]
    pub external_ipaddress: Option<String>,

/// 
    #[serde(rename = "ExternalPort")]
    pub external_port: Option<u16>,

/// 
    #[serde(rename = "InternalIPAddress")]
    pub internal_ipaddress: Option<String>,

/// 
    #[serde(rename = "InternalPort")]
    pub internal_port: Option<u16>,

/// 
    #[serde(rename = "InternalRoutingDomainId")]
    pub internal_routing_domain_id: Option<String>,

/// 
    #[serde(rename = "NatName")]
    pub nat_name: Option<String>,

/// 
    #[serde(rename = "Protocol")]
    pub protocol: Option<u32>,

/// 
    #[serde(rename = "RemoteExternalIPAddressPrefix")]
    pub remote_external_ipaddress_prefix: Option<String>,

/// 
    #[serde(rename = "StaticMappingID")]
    pub static_mapping_id: Option<u32>,
}

impl MSFT_NetNatStaticMapping {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            active: None,
            external_ipaddress: None,
            external_port: None,
            internal_ipaddress: None,
            internal_port: None,
            internal_routing_domain_id: None,
            nat_name: None,
            protocol: None,
            remote_external_ipaddress_prefix: None,
            static_mapping_id: None,
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

    /// Sets the value of ExternalIPAddress
    pub fn set_external_ipaddress(&mut self, value: String) {
        self.external_ipaddress = Some(value);
    }

    /// Gets the value of ExternalIPAddress
    pub fn get_external_ipaddress(&self) -> Option<&String> {
        self.external_ipaddress.as_ref()
    }

    /// Sets the value of ExternalPort
    pub fn set_external_port(&mut self, value: u16) {
        self.external_port = Some(value);
    }

    /// Gets the value of ExternalPort
    pub fn get_external_port(&self) -> Option<&u16> {
        self.external_port.as_ref()
    }

    /// Sets the value of InternalIPAddress
    pub fn set_internal_ipaddress(&mut self, value: String) {
        self.internal_ipaddress = Some(value);
    }

    /// Gets the value of InternalIPAddress
    pub fn get_internal_ipaddress(&self) -> Option<&String> {
        self.internal_ipaddress.as_ref()
    }

    /// Sets the value of InternalPort
    pub fn set_internal_port(&mut self, value: u16) {
        self.internal_port = Some(value);
    }

    /// Gets the value of InternalPort
    pub fn get_internal_port(&self) -> Option<&u16> {
        self.internal_port.as_ref()
    }

    /// Sets the value of InternalRoutingDomainId
    pub fn set_internal_routing_domain_id(&mut self, value: String) {
        self.internal_routing_domain_id = Some(value);
    }

    /// Gets the value of InternalRoutingDomainId
    pub fn get_internal_routing_domain_id(&self) -> Option<&String> {
        self.internal_routing_domain_id.as_ref()
    }

    /// Sets the value of NatName
    pub fn set_nat_name(&mut self, value: String) {
        self.nat_name = Some(value);
    }

    /// Gets the value of NatName
    pub fn get_nat_name(&self) -> Option<&String> {
        self.nat_name.as_ref()
    }

    /// Sets the value of Protocol
    pub fn set_protocol(&mut self, value: u32) {
        self.protocol = Some(value);
    }

    /// Gets the value of Protocol
    pub fn get_protocol(&self) -> Option<&u32> {
        self.protocol.as_ref()
    }

    /// Sets the value of RemoteExternalIPAddressPrefix
    pub fn set_remote_external_ipaddress_prefix(&mut self, value: String) {
        self.remote_external_ipaddress_prefix = Some(value);
    }

    /// Gets the value of RemoteExternalIPAddressPrefix
    pub fn get_remote_external_ipaddress_prefix(&self) -> Option<&String> {
        self.remote_external_ipaddress_prefix.as_ref()
    }

    /// Sets the value of StaticMappingID
    pub fn set_static_mapping_id(&mut self, value: u32) {
        self.static_mapping_id = Some(value);
    }

    /// Gets the value of StaticMappingID
    pub fn get_static_mapping_id(&self) -> Option<&u32> {
        self.static_mapping_id.as_ref()
    }
}

