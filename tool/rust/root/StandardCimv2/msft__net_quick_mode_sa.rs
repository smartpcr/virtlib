// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetQuickModeSA struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetQuickModeSA {
    #[serde(flatten)]
    pub base: CIM_IPsecSAEndpoint,

/// 
    #[serde(rename = "EmTargetName")]
    pub em_target_name: Option<String>,

/// 
    #[serde(rename = "ExplicitCredentials")]
    pub explicit_credentials: Option<u64>,

/// 
    #[serde(rename = "FirstCipherAlgorithm")]
    pub first_cipher_algorithm: Option<u32>,

/// 
    #[serde(rename = "FirstIntegrityAlgorithm")]
    pub first_integrity_algorithm: Option<u32>,

/// 
    #[serde(rename = "FirstTransformType")]
    pub first_transform_type: Option<u32>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "InterfaceAlias")]
    pub interface_alias: Option<String>,

/// 
    #[serde(rename = "IpProtocol")]
    pub ip_protocol: Option<u8>,

/// 
    #[serde(rename = "LifetimePackets")]
    pub lifetime_packets: Option<u64>,

/// 
    #[serde(rename = "LocalEndpoint")]
    pub local_endpoint: Option<String>,

/// 
    #[serde(rename = "LocalPort")]
    pub local_port: Option<u16>,

/// 
    #[serde(rename = "LocalUdpEncapsulationPort")]
    pub local_udp_encapsulation_port: Option<u16>,

/// 
    #[serde(rename = "MmSaId")]
    pub mm_sa_id: Option<u64>,

/// 
    #[serde(rename = "MmTargetName")]
    pub mm_target_name: Option<String>,

/// 
    #[serde(rename = "NapContext")]
    pub nap_context: Option<u32>,

/// 
    #[serde(rename = "NdAllowClearTimeoutSeconds")]
    pub nd_allow_clear_timeout_seconds: Option<u32>,

/// 
    #[serde(rename = "PeerV4PrivateAddress")]
    pub peer_v4_private_address: Option<String>,

/// 
    #[serde(rename = "PfsGroupId")]
    pub pfs_group_id: Option<u32>,

/// 
    #[serde(rename = "QmSaId")]
    pub qm_sa_id: Option<u32>,

/// 
    #[serde(rename = "QuickModeFilterId")]
    pub quick_mode_filter_id: Option<u64>,

/// 
    #[serde(rename = "RealIfProfileId")]
    pub real_if_profile_id: Option<u64>,

/// 
    #[serde(rename = "RemoteEndpoint")]
    pub remote_endpoint: Option<String>,

/// 
    #[serde(rename = "RemotePort")]
    pub remote_port: Option<u16>,

/// 
    #[serde(rename = "RemoteUdpEncapsulationPort")]
    pub remote_udp_encapsulation_port: Option<u16>,

/// 
    #[serde(rename = "SecondCipherAlgorithm")]
    pub second_cipher_algorithm: Option<u32>,

/// 
    #[serde(rename = "SecondIntegrityAlgorithm")]
    pub second_integrity_algorithm: Option<u32>,

/// 
    #[serde(rename = "SecondSPI")]
    pub second_spi: Option<u32>,

/// 
    #[serde(rename = "SecondTransformType")]
    pub second_transform_type: Option<u32>,

/// 
    #[serde(rename = "TrafficLuid")]
    pub traffic_luid: Option<u64>,

/// 
    #[serde(rename = "TrafficSelectorId")]
    pub traffic_selector_id: Option<u64>,

/// 
    #[serde(rename = "TransportLayerFilterName")]
    pub transport_layer_filter_name: Option<String>,

/// 
    #[serde(rename = "VirtualIfTunnelId")]
    pub virtual_if_tunnel_id: Option<u64>,
}

impl MSFT_NetQuickModeSA {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_IPsecSAEndpoint::new(),
            em_target_name: None,
            explicit_credentials: None,
            first_cipher_algorithm: None,
            first_integrity_algorithm: None,
            first_transform_type: None,
            flags: None,
            interface_alias: None,
            ip_protocol: None,
            lifetime_packets: None,
            local_endpoint: None,
            local_port: None,
            local_udp_encapsulation_port: None,
            mm_sa_id: None,
            mm_target_name: None,
            nap_context: None,
            nd_allow_clear_timeout_seconds: None,
            peer_v4_private_address: None,
            pfs_group_id: None,
            qm_sa_id: None,
            quick_mode_filter_id: None,
            real_if_profile_id: None,
            remote_endpoint: None,
            remote_port: None,
            remote_udp_encapsulation_port: None,
            second_cipher_algorithm: None,
            second_integrity_algorithm: None,
            second_spi: None,
            second_transform_type: None,
            traffic_luid: None,
            traffic_selector_id: None,
            transport_layer_filter_name: None,
            virtual_if_tunnel_id: None,
        }
    }


    /// Sets the value of EmTargetName
    pub fn set_em_target_name(&mut self, value: String) {
        self.em_target_name = Some(value);
    }

    /// Gets the value of EmTargetName
    pub fn get_em_target_name(&self) -> Option<&String> {
        self.em_target_name.as_ref()
    }

    /// Sets the value of ExplicitCredentials
    pub fn set_explicit_credentials(&mut self, value: u64) {
        self.explicit_credentials = Some(value);
    }

    /// Gets the value of ExplicitCredentials
    pub fn get_explicit_credentials(&self) -> Option<&u64> {
        self.explicit_credentials.as_ref()
    }

    /// Sets the value of FirstCipherAlgorithm
    pub fn set_first_cipher_algorithm(&mut self, value: u32) {
        self.first_cipher_algorithm = Some(value);
    }

    /// Gets the value of FirstCipherAlgorithm
    pub fn get_first_cipher_algorithm(&self) -> Option<&u32> {
        self.first_cipher_algorithm.as_ref()
    }

    /// Sets the value of FirstIntegrityAlgorithm
    pub fn set_first_integrity_algorithm(&mut self, value: u32) {
        self.first_integrity_algorithm = Some(value);
    }

    /// Gets the value of FirstIntegrityAlgorithm
    pub fn get_first_integrity_algorithm(&self) -> Option<&u32> {
        self.first_integrity_algorithm.as_ref()
    }

    /// Sets the value of FirstTransformType
    pub fn set_first_transform_type(&mut self, value: u32) {
        self.first_transform_type = Some(value);
    }

    /// Gets the value of FirstTransformType
    pub fn get_first_transform_type(&self) -> Option<&u32> {
        self.first_transform_type.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of InterfaceAlias
    pub fn set_interface_alias(&mut self, value: String) {
        self.interface_alias = Some(value);
    }

    /// Gets the value of InterfaceAlias
    pub fn get_interface_alias(&self) -> Option<&String> {
        self.interface_alias.as_ref()
    }

    /// Sets the value of IpProtocol
    pub fn set_ip_protocol(&mut self, value: u8) {
        self.ip_protocol = Some(value);
    }

    /// Gets the value of IpProtocol
    pub fn get_ip_protocol(&self) -> Option<&u8> {
        self.ip_protocol.as_ref()
    }

    /// Sets the value of LifetimePackets
    pub fn set_lifetime_packets(&mut self, value: u64) {
        self.lifetime_packets = Some(value);
    }

    /// Gets the value of LifetimePackets
    pub fn get_lifetime_packets(&self) -> Option<&u64> {
        self.lifetime_packets.as_ref()
    }

    /// Sets the value of LocalEndpoint
    pub fn set_local_endpoint(&mut self, value: String) {
        self.local_endpoint = Some(value);
    }

    /// Gets the value of LocalEndpoint
    pub fn get_local_endpoint(&self) -> Option<&String> {
        self.local_endpoint.as_ref()
    }

    /// Sets the value of LocalPort
    pub fn set_local_port(&mut self, value: u16) {
        self.local_port = Some(value);
    }

    /// Gets the value of LocalPort
    pub fn get_local_port(&self) -> Option<&u16> {
        self.local_port.as_ref()
    }

    /// Sets the value of LocalUdpEncapsulationPort
    pub fn set_local_udp_encapsulation_port(&mut self, value: u16) {
        self.local_udp_encapsulation_port = Some(value);
    }

    /// Gets the value of LocalUdpEncapsulationPort
    pub fn get_local_udp_encapsulation_port(&self) -> Option<&u16> {
        self.local_udp_encapsulation_port.as_ref()
    }

    /// Sets the value of MmSaId
    pub fn set_mm_sa_id(&mut self, value: u64) {
        self.mm_sa_id = Some(value);
    }

    /// Gets the value of MmSaId
    pub fn get_mm_sa_id(&self) -> Option<&u64> {
        self.mm_sa_id.as_ref()
    }

    /// Sets the value of MmTargetName
    pub fn set_mm_target_name(&mut self, value: String) {
        self.mm_target_name = Some(value);
    }

    /// Gets the value of MmTargetName
    pub fn get_mm_target_name(&self) -> Option<&String> {
        self.mm_target_name.as_ref()
    }

    /// Sets the value of NapContext
    pub fn set_nap_context(&mut self, value: u32) {
        self.nap_context = Some(value);
    }

    /// Gets the value of NapContext
    pub fn get_nap_context(&self) -> Option<&u32> {
        self.nap_context.as_ref()
    }

    /// Sets the value of NdAllowClearTimeoutSeconds
    pub fn set_nd_allow_clear_timeout_seconds(&mut self, value: u32) {
        self.nd_allow_clear_timeout_seconds = Some(value);
    }

    /// Gets the value of NdAllowClearTimeoutSeconds
    pub fn get_nd_allow_clear_timeout_seconds(&self) -> Option<&u32> {
        self.nd_allow_clear_timeout_seconds.as_ref()
    }

    /// Sets the value of PeerV4PrivateAddress
    pub fn set_peer_v4_private_address(&mut self, value: String) {
        self.peer_v4_private_address = Some(value);
    }

    /// Gets the value of PeerV4PrivateAddress
    pub fn get_peer_v4_private_address(&self) -> Option<&String> {
        self.peer_v4_private_address.as_ref()
    }

    /// Sets the value of PfsGroupId
    pub fn set_pfs_group_id(&mut self, value: u32) {
        self.pfs_group_id = Some(value);
    }

    /// Gets the value of PfsGroupId
    pub fn get_pfs_group_id(&self) -> Option<&u32> {
        self.pfs_group_id.as_ref()
    }

    /// Sets the value of QmSaId
    pub fn set_qm_sa_id(&mut self, value: u32) {
        self.qm_sa_id = Some(value);
    }

    /// Gets the value of QmSaId
    pub fn get_qm_sa_id(&self) -> Option<&u32> {
        self.qm_sa_id.as_ref()
    }

    /// Sets the value of QuickModeFilterId
    pub fn set_quick_mode_filter_id(&mut self, value: u64) {
        self.quick_mode_filter_id = Some(value);
    }

    /// Gets the value of QuickModeFilterId
    pub fn get_quick_mode_filter_id(&self) -> Option<&u64> {
        self.quick_mode_filter_id.as_ref()
    }

    /// Sets the value of RealIfProfileId
    pub fn set_real_if_profile_id(&mut self, value: u64) {
        self.real_if_profile_id = Some(value);
    }

    /// Gets the value of RealIfProfileId
    pub fn get_real_if_profile_id(&self) -> Option<&u64> {
        self.real_if_profile_id.as_ref()
    }

    /// Sets the value of RemoteEndpoint
    pub fn set_remote_endpoint(&mut self, value: String) {
        self.remote_endpoint = Some(value);
    }

    /// Gets the value of RemoteEndpoint
    pub fn get_remote_endpoint(&self) -> Option<&String> {
        self.remote_endpoint.as_ref()
    }

    /// Sets the value of RemotePort
    pub fn set_remote_port(&mut self, value: u16) {
        self.remote_port = Some(value);
    }

    /// Gets the value of RemotePort
    pub fn get_remote_port(&self) -> Option<&u16> {
        self.remote_port.as_ref()
    }

    /// Sets the value of RemoteUdpEncapsulationPort
    pub fn set_remote_udp_encapsulation_port(&mut self, value: u16) {
        self.remote_udp_encapsulation_port = Some(value);
    }

    /// Gets the value of RemoteUdpEncapsulationPort
    pub fn get_remote_udp_encapsulation_port(&self) -> Option<&u16> {
        self.remote_udp_encapsulation_port.as_ref()
    }

    /// Sets the value of SecondCipherAlgorithm
    pub fn set_second_cipher_algorithm(&mut self, value: u32) {
        self.second_cipher_algorithm = Some(value);
    }

    /// Gets the value of SecondCipherAlgorithm
    pub fn get_second_cipher_algorithm(&self) -> Option<&u32> {
        self.second_cipher_algorithm.as_ref()
    }

    /// Sets the value of SecondIntegrityAlgorithm
    pub fn set_second_integrity_algorithm(&mut self, value: u32) {
        self.second_integrity_algorithm = Some(value);
    }

    /// Gets the value of SecondIntegrityAlgorithm
    pub fn get_second_integrity_algorithm(&self) -> Option<&u32> {
        self.second_integrity_algorithm.as_ref()
    }

    /// Sets the value of SecondSPI
    pub fn set_second_spi(&mut self, value: u32) {
        self.second_spi = Some(value);
    }

    /// Gets the value of SecondSPI
    pub fn get_second_spi(&self) -> Option<&u32> {
        self.second_spi.as_ref()
    }

    /// Sets the value of SecondTransformType
    pub fn set_second_transform_type(&mut self, value: u32) {
        self.second_transform_type = Some(value);
    }

    /// Gets the value of SecondTransformType
    pub fn get_second_transform_type(&self) -> Option<&u32> {
        self.second_transform_type.as_ref()
    }

    /// Sets the value of TrafficLuid
    pub fn set_traffic_luid(&mut self, value: u64) {
        self.traffic_luid = Some(value);
    }

    /// Gets the value of TrafficLuid
    pub fn get_traffic_luid(&self) -> Option<&u64> {
        self.traffic_luid.as_ref()
    }

    /// Sets the value of TrafficSelectorId
    pub fn set_traffic_selector_id(&mut self, value: u64) {
        self.traffic_selector_id = Some(value);
    }

    /// Gets the value of TrafficSelectorId
    pub fn get_traffic_selector_id(&self) -> Option<&u64> {
        self.traffic_selector_id.as_ref()
    }

    /// Sets the value of TransportLayerFilterName
    pub fn set_transport_layer_filter_name(&mut self, value: String) {
        self.transport_layer_filter_name = Some(value);
    }

    /// Gets the value of TransportLayerFilterName
    pub fn get_transport_layer_filter_name(&self) -> Option<&String> {
        self.transport_layer_filter_name.as_ref()
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

