// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// VpnCommonConfig struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VpnCommonConfig {

/// 
    #[serde(rename = "ConnectionStatus")]
    pub connection_status: Option<String>,

/// 
    #[serde(rename = "DnsSuffix")]
    pub dns_suffix: Option<String>,

/// 
    #[serde(rename = "Guid")]
    pub guid: Option<String>,

/// 
    #[serde(rename = "IdleDisconnectSeconds")]
    pub idle_disconnect_seconds: Option<u32>,

/// 
    #[serde(rename = "IsAutoTriggerEnabled")]
    pub is_auto_trigger_enabled: Option<bool>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "ProfileType")]
    pub profile_type: Option<String>,

/// 
    #[serde(rename = "ProvisioningAuthority")]
    pub provisioning_authority: Option<String>,

/// 
    #[serde(rename = "Proxy")]
    pub proxy: Option<VpnConnectionProxy>,

/// 
    #[serde(rename = "RememberCredential")]
    pub remember_credential: Option<bool>,

/// 
    #[serde(rename = "Routes")]
    pub routes: Vec<MSFT_NetRoute>,

/// 
    #[serde(rename = "ServerAddress")]
    pub server_address: Option<String>,

/// 
    #[serde(rename = "ServerList")]
    pub server_list: Vec<VpnServerAddress>,

/// 
    #[serde(rename = "SplitTunneling")]
    pub split_tunneling: Option<bool>,

/// 
    #[serde(rename = "VpnTrigger")]
    pub vpn_trigger: Option<VpnConnectionTrigger>,
}

impl VpnCommonConfig {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            connection_status: None,
            dns_suffix: None,
            guid: None,
            idle_disconnect_seconds: None,
            is_auto_trigger_enabled: None,
            name: None,
            profile_type: None,
            provisioning_authority: None,
            proxy: None,
            remember_credential: None,
            routes: Vec::new(),
            server_address: None,
            server_list: Vec::new(),
            split_tunneling: None,
            vpn_trigger: None,
        }
    }


    /// Sets the value of ConnectionStatus
    pub fn set_connection_status(&mut self, value: String) {
        self.connection_status = Some(value);
    }

    /// Gets the value of ConnectionStatus
    pub fn get_connection_status(&self) -> Option<&String> {
        self.connection_status.as_ref()
    }

    /// Sets the value of DnsSuffix
    pub fn set_dns_suffix(&mut self, value: String) {
        self.dns_suffix = Some(value);
    }

    /// Gets the value of DnsSuffix
    pub fn get_dns_suffix(&self) -> Option<&String> {
        self.dns_suffix.as_ref()
    }

    /// Sets the value of Guid
    pub fn set_guid(&mut self, value: String) {
        self.guid = Some(value);
    }

    /// Gets the value of Guid
    pub fn get_guid(&self) -> Option<&String> {
        self.guid.as_ref()
    }

    /// Sets the value of IdleDisconnectSeconds
    pub fn set_idle_disconnect_seconds(&mut self, value: u32) {
        self.idle_disconnect_seconds = Some(value);
    }

    /// Gets the value of IdleDisconnectSeconds
    pub fn get_idle_disconnect_seconds(&self) -> Option<&u32> {
        self.idle_disconnect_seconds.as_ref()
    }

    /// Sets the value of IsAutoTriggerEnabled
    pub fn set_is_auto_trigger_enabled(&mut self, value: bool) {
        self.is_auto_trigger_enabled = Some(value);
    }

    /// Gets the value of IsAutoTriggerEnabled
    pub fn get_is_auto_trigger_enabled(&self) -> Option<&bool> {
        self.is_auto_trigger_enabled.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of ProfileType
    pub fn set_profile_type(&mut self, value: String) {
        self.profile_type = Some(value);
    }

    /// Gets the value of ProfileType
    pub fn get_profile_type(&self) -> Option<&String> {
        self.profile_type.as_ref()
    }

    /// Sets the value of ProvisioningAuthority
    pub fn set_provisioning_authority(&mut self, value: String) {
        self.provisioning_authority = Some(value);
    }

    /// Gets the value of ProvisioningAuthority
    pub fn get_provisioning_authority(&self) -> Option<&String> {
        self.provisioning_authority.as_ref()
    }

    /// Sets the value of Proxy
    pub fn set_proxy(&mut self, value: VpnConnectionProxy) {
        self.proxy = Some(value);
    }

    /// Gets the value of Proxy
    pub fn get_proxy(&self) -> Option<&VpnConnectionProxy> {
        self.proxy.as_ref()
    }

    /// Sets the value of RememberCredential
    pub fn set_remember_credential(&mut self, value: bool) {
        self.remember_credential = Some(value);
    }

    /// Gets the value of RememberCredential
    pub fn get_remember_credential(&self) -> Option<&bool> {
        self.remember_credential.as_ref()
    }

    /// Sets the value of Routes
    pub fn set_routes(&mut self, value: Vec<MSFT_NetRoute>) {
        self.routes = value;
    }

    /// Gets the value of Routes
    pub fn get_routes(&self) -> &Vec<MSFT_NetRoute> {
        &self.routes
    }

    /// Sets the value of ServerAddress
    pub fn set_server_address(&mut self, value: String) {
        self.server_address = Some(value);
    }

    /// Gets the value of ServerAddress
    pub fn get_server_address(&self) -> Option<&String> {
        self.server_address.as_ref()
    }

    /// Sets the value of ServerList
    pub fn set_server_list(&mut self, value: Vec<VpnServerAddress>) {
        self.server_list = value;
    }

    /// Gets the value of ServerList
    pub fn get_server_list(&self) -> &Vec<VpnServerAddress> {
        &self.server_list
    }

    /// Sets the value of SplitTunneling
    pub fn set_split_tunneling(&mut self, value: bool) {
        self.split_tunneling = Some(value);
    }

    /// Gets the value of SplitTunneling
    pub fn get_split_tunneling(&self) -> Option<&bool> {
        self.split_tunneling.as_ref()
    }

    /// Sets the value of VpnTrigger
    pub fn set_vpn_trigger(&mut self, value: VpnConnectionTrigger) {
        self.vpn_trigger = Some(value);
    }

    /// Gets the value of VpnTrigger
    pub fn get_vpn_trigger(&self) -> Option<&VpnConnectionTrigger> {
        self.vpn_trigger.as_ref()
    }
}

