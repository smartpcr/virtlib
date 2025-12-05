// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetMainModeSA struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetMainModeSA {
    #[serde(flatten)]
    pub base: CIM_IKESAEndpoint,

/// 
    #[serde(rename = "ExtendedFilterId")]
    pub extended_filter_id: Option<u64>,

/// 
    #[serde(rename = "IkePolicyKey")]
    pub ike_policy_key: Option<String>,

/// 
    #[serde(rename = "KeyModule")]
    pub key_module: Option<u16>,

/// 
    #[serde(rename = "LocalEndpoint")]
    pub local_endpoint: Option<String>,

/// 
    #[serde(rename = "LocalFirstId")]
    pub local_first_id: Option<MSFT_NetIPsecIdentity>,

/// 
    #[serde(rename = "LocalSecondId")]
    pub local_second_id: Option<MSFT_NetIPsecIdentity>,

/// 
    #[serde(rename = "LocalUdpEncapsulationPort")]
    pub local_udp_encapsulation_port: Option<u16>,

/// 
    #[serde(rename = "MaxQMSAs")]
    pub max_qmsas: Option<u32>,

/// 
    #[serde(rename = "OtherGroupId")]
    pub other_group_id: Option<String>,

/// 
    #[serde(rename = "RemoteEndpoint")]
    pub remote_endpoint: Option<String>,

/// 
    #[serde(rename = "RemoteFirstId")]
    pub remote_first_id: Option<MSFT_NetIPsecIdentity>,

/// 
    #[serde(rename = "RemoteSecondId")]
    pub remote_second_id: Option<MSFT_NetIPsecIdentity>,

/// 
    #[serde(rename = "RemoteUdpEncapsulationPort")]
    pub remote_udp_encapsulation_port: Option<u16>,

/// 
    #[serde(rename = "VirtualIfTunnelId")]
    pub virtual_if_tunnel_id: Option<u64>,
}

impl MSFT_NetMainModeSA {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_IKESAEndpoint::new(),
            extended_filter_id: None,
            ike_policy_key: None,
            key_module: None,
            local_endpoint: None,
            local_first_id: None,
            local_second_id: None,
            local_udp_encapsulation_port: None,
            max_qmsas: None,
            other_group_id: None,
            remote_endpoint: None,
            remote_first_id: None,
            remote_second_id: None,
            remote_udp_encapsulation_port: None,
            virtual_if_tunnel_id: None,
        }
    }


    /// Sets the value of ExtendedFilterId
    pub fn set_extended_filter_id(&mut self, value: u64) {
        self.extended_filter_id = Some(value);
    }

    /// Gets the value of ExtendedFilterId
    pub fn get_extended_filter_id(&self) -> Option<&u64> {
        self.extended_filter_id.as_ref()
    }

    /// Sets the value of IkePolicyKey
    pub fn set_ike_policy_key(&mut self, value: String) {
        self.ike_policy_key = Some(value);
    }

    /// Gets the value of IkePolicyKey
    pub fn get_ike_policy_key(&self) -> Option<&String> {
        self.ike_policy_key.as_ref()
    }

    /// Sets the value of KeyModule
    pub fn set_key_module(&mut self, value: u16) {
        self.key_module = Some(value);
    }

    /// Gets the value of KeyModule
    pub fn get_key_module(&self) -> Option<&u16> {
        self.key_module.as_ref()
    }

    /// Sets the value of LocalEndpoint
    pub fn set_local_endpoint(&mut self, value: String) {
        self.local_endpoint = Some(value);
    }

    /// Gets the value of LocalEndpoint
    pub fn get_local_endpoint(&self) -> Option<&String> {
        self.local_endpoint.as_ref()
    }

    /// Sets the value of LocalFirstId
    pub fn set_local_first_id(&mut self, value: MSFT_NetIPsecIdentity) {
        self.local_first_id = Some(value);
    }

    /// Gets the value of LocalFirstId
    pub fn get_local_first_id(&self) -> Option<&MSFT_NetIPsecIdentity> {
        self.local_first_id.as_ref()
    }

    /// Sets the value of LocalSecondId
    pub fn set_local_second_id(&mut self, value: MSFT_NetIPsecIdentity) {
        self.local_second_id = Some(value);
    }

    /// Gets the value of LocalSecondId
    pub fn get_local_second_id(&self) -> Option<&MSFT_NetIPsecIdentity> {
        self.local_second_id.as_ref()
    }

    /// Sets the value of LocalUdpEncapsulationPort
    pub fn set_local_udp_encapsulation_port(&mut self, value: u16) {
        self.local_udp_encapsulation_port = Some(value);
    }

    /// Gets the value of LocalUdpEncapsulationPort
    pub fn get_local_udp_encapsulation_port(&self) -> Option<&u16> {
        self.local_udp_encapsulation_port.as_ref()
    }

    /// Sets the value of MaxQMSAs
    pub fn set_max_qmsas(&mut self, value: u32) {
        self.max_qmsas = Some(value);
    }

    /// Gets the value of MaxQMSAs
    pub fn get_max_qmsas(&self) -> Option<&u32> {
        self.max_qmsas.as_ref()
    }

    /// Sets the value of OtherGroupId
    pub fn set_other_group_id(&mut self, value: String) {
        self.other_group_id = Some(value);
    }

    /// Gets the value of OtherGroupId
    pub fn get_other_group_id(&self) -> Option<&String> {
        self.other_group_id.as_ref()
    }

    /// Sets the value of RemoteEndpoint
    pub fn set_remote_endpoint(&mut self, value: String) {
        self.remote_endpoint = Some(value);
    }

    /// Gets the value of RemoteEndpoint
    pub fn get_remote_endpoint(&self) -> Option<&String> {
        self.remote_endpoint.as_ref()
    }

    /// Sets the value of RemoteFirstId
    pub fn set_remote_first_id(&mut self, value: MSFT_NetIPsecIdentity) {
        self.remote_first_id = Some(value);
    }

    /// Gets the value of RemoteFirstId
    pub fn get_remote_first_id(&self) -> Option<&MSFT_NetIPsecIdentity> {
        self.remote_first_id.as_ref()
    }

    /// Sets the value of RemoteSecondId
    pub fn set_remote_second_id(&mut self, value: MSFT_NetIPsecIdentity) {
        self.remote_second_id = Some(value);
    }

    /// Gets the value of RemoteSecondId
    pub fn get_remote_second_id(&self) -> Option<&MSFT_NetIPsecIdentity> {
        self.remote_second_id.as_ref()
    }

    /// Sets the value of RemoteUdpEncapsulationPort
    pub fn set_remote_udp_encapsulation_port(&mut self, value: u16) {
        self.remote_udp_encapsulation_port = Some(value);
    }

    /// Gets the value of RemoteUdpEncapsulationPort
    pub fn get_remote_udp_encapsulation_port(&self) -> Option<&u16> {
        self.remote_udp_encapsulation_port.as_ref()
    }

    /// Sets the value of VirtualIfTunnelId
    pub fn set_virtual_if_tunnel_id(&mut self, value: u64) {
        self.virtual_if_tunnel_id = Some(value);
    }

    /// Gets the value of VirtualIfTunnelId
    pub fn get_virtual_if_tunnel_id(&self) -> Option<&u64> {
        self.virtual_if_tunnel_id.as_ref()
    }
}

