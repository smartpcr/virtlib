// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetSecuritySettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetSecuritySettingData {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "AllowIPsecThroughNAT")]
    pub allow_ipsec_through_nat: Option<u16>,

/// 
    #[serde(rename = "CertValidationLevel")]
    pub cert_validation_level: Option<u16>,

/// 
    #[serde(rename = "EnablePacketQueuing")]
    pub enable_packet_queuing: Option<u16>,

/// 
    #[serde(rename = "EnableStatefulFtp")]
    pub enable_stateful_ftp: Option<u16>,

/// 
    #[serde(rename = "EnableStatefulPptp")]
    pub enable_stateful_pptp: Option<u16>,

/// 
    #[serde(rename = "Exemptions")]
    pub exemptions: Option<u32>,

/// 
    #[serde(rename = "KeyEncoding")]
    pub key_encoding: Option<u16>,

/// 
    #[serde(rename = "MaxSAIdleTimeSeconds")]
    pub max_saidle_time_seconds: Option<u32>,

/// 
    #[serde(rename = "Profile")]
    pub profile: Option<u16>,

/// 
    #[serde(rename = "RemoteMachineTransportAuthorizationList")]
    pub remote_machine_transport_authorization_list: Option<String>,

/// 
    #[serde(rename = "RemoteMachineTunnelAuthorizationList")]
    pub remote_machine_tunnel_authorization_list: Option<String>,

/// 
    #[serde(rename = "RemoteUserTransportAuthorizationList")]
    pub remote_user_transport_authorization_list: Option<String>,

/// 
    #[serde(rename = "RemoteUserTunnelAuthorizationList")]
    pub remote_user_tunnel_authorization_list: Option<String>,

/// 
    #[serde(rename = "RequireFullAuthSupport")]
    pub require_full_auth_support: Option<u16>,
}

impl MSFT_NetSecuritySettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            allow_ipsec_through_nat: None,
            cert_validation_level: None,
            enable_packet_queuing: None,
            enable_stateful_ftp: None,
            enable_stateful_pptp: None,
            exemptions: None,
            key_encoding: None,
            max_saidle_time_seconds: None,
            profile: None,
            remote_machine_transport_authorization_list: None,
            remote_machine_tunnel_authorization_list: None,
            remote_user_transport_authorization_list: None,
            remote_user_tunnel_authorization_list: None,
            require_full_auth_support: None,
        }
    }


    /// Sets the value of AllowIPsecThroughNAT
    pub fn set_allow_ipsec_through_nat(&mut self, value: u16) {
        self.allow_ipsec_through_nat = Some(value);
    }

    /// Gets the value of AllowIPsecThroughNAT
    pub fn get_allow_ipsec_through_nat(&self) -> Option<&u16> {
        self.allow_ipsec_through_nat.as_ref()
    }

    /// Sets the value of CertValidationLevel
    pub fn set_cert_validation_level(&mut self, value: u16) {
        self.cert_validation_level = Some(value);
    }

    /// Gets the value of CertValidationLevel
    pub fn get_cert_validation_level(&self) -> Option<&u16> {
        self.cert_validation_level.as_ref()
    }

    /// Sets the value of EnablePacketQueuing
    pub fn set_enable_packet_queuing(&mut self, value: u16) {
        self.enable_packet_queuing = Some(value);
    }

    /// Gets the value of EnablePacketQueuing
    pub fn get_enable_packet_queuing(&self) -> Option<&u16> {
        self.enable_packet_queuing.as_ref()
    }

    /// Sets the value of EnableStatefulFtp
    pub fn set_enable_stateful_ftp(&mut self, value: u16) {
        self.enable_stateful_ftp = Some(value);
    }

    /// Gets the value of EnableStatefulFtp
    pub fn get_enable_stateful_ftp(&self) -> Option<&u16> {
        self.enable_stateful_ftp.as_ref()
    }

    /// Sets the value of EnableStatefulPptp
    pub fn set_enable_stateful_pptp(&mut self, value: u16) {
        self.enable_stateful_pptp = Some(value);
    }

    /// Gets the value of EnableStatefulPptp
    pub fn get_enable_stateful_pptp(&self) -> Option<&u16> {
        self.enable_stateful_pptp.as_ref()
    }

    /// Sets the value of Exemptions
    pub fn set_exemptions(&mut self, value: u32) {
        self.exemptions = Some(value);
    }

    /// Gets the value of Exemptions
    pub fn get_exemptions(&self) -> Option<&u32> {
        self.exemptions.as_ref()
    }

    /// Sets the value of KeyEncoding
    pub fn set_key_encoding(&mut self, value: u16) {
        self.key_encoding = Some(value);
    }

    /// Gets the value of KeyEncoding
    pub fn get_key_encoding(&self) -> Option<&u16> {
        self.key_encoding.as_ref()
    }

    /// Sets the value of MaxSAIdleTimeSeconds
    pub fn set_max_saidle_time_seconds(&mut self, value: u32) {
        self.max_saidle_time_seconds = Some(value);
    }

    /// Gets the value of MaxSAIdleTimeSeconds
    pub fn get_max_saidle_time_seconds(&self) -> Option<&u32> {
        self.max_saidle_time_seconds.as_ref()
    }

    /// Sets the value of Profile
    pub fn set_profile(&mut self, value: u16) {
        self.profile = Some(value);
    }

    /// Gets the value of Profile
    pub fn get_profile(&self) -> Option<&u16> {
        self.profile.as_ref()
    }

    /// Sets the value of RemoteMachineTransportAuthorizationList
    pub fn set_remote_machine_transport_authorization_list(&mut self, value: String) {
        self.remote_machine_transport_authorization_list = Some(value);
    }

    /// Gets the value of RemoteMachineTransportAuthorizationList
    pub fn get_remote_machine_transport_authorization_list(&self) -> Option<&String> {
        self.remote_machine_transport_authorization_list.as_ref()
    }

    /// Sets the value of RemoteMachineTunnelAuthorizationList
    pub fn set_remote_machine_tunnel_authorization_list(&mut self, value: String) {
        self.remote_machine_tunnel_authorization_list = Some(value);
    }

    /// Gets the value of RemoteMachineTunnelAuthorizationList
    pub fn get_remote_machine_tunnel_authorization_list(&self) -> Option<&String> {
        self.remote_machine_tunnel_authorization_list.as_ref()
    }

    /// Sets the value of RemoteUserTransportAuthorizationList
    pub fn set_remote_user_transport_authorization_list(&mut self, value: String) {
        self.remote_user_transport_authorization_list = Some(value);
    }

    /// Gets the value of RemoteUserTransportAuthorizationList
    pub fn get_remote_user_transport_authorization_list(&self) -> Option<&String> {
        self.remote_user_transport_authorization_list.as_ref()
    }

    /// Sets the value of RemoteUserTunnelAuthorizationList
    pub fn set_remote_user_tunnel_authorization_list(&mut self, value: String) {
        self.remote_user_tunnel_authorization_list = Some(value);
    }

    /// Gets the value of RemoteUserTunnelAuthorizationList
    pub fn get_remote_user_tunnel_authorization_list(&self) -> Option<&String> {
        self.remote_user_tunnel_authorization_list.as_ref()
    }

    /// Sets the value of RequireFullAuthSupport
    pub fn set_require_full_auth_support(&mut self, value: u16) {
        self.require_full_auth_support = Some(value);
    }

    /// Gets the value of RequireFullAuthSupport
    pub fn get_require_full_auth_support(&self) -> Option<&u16> {
        self.require_full_auth_support.as_ref()
    }
}

