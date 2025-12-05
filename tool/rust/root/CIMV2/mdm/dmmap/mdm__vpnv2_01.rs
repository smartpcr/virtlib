// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_VPNv2_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_VPNv2_01 {

/// 
    #[serde(rename = "AlwaysOn")]
    pub always_on: Option<bool>,

/// 
    #[serde(rename = "AlwaysOnActive")]
    pub always_on_active: Option<bool>,

/// 
    #[serde(rename = "ByPassForLocal")]
    pub by_pass_for_local: Option<bool>,

/// 
    #[serde(rename = "DataEncryption")]
    pub data_encryption: Option<String>,

/// 
    #[serde(rename = "DeviceTunnel")]
    pub device_tunnel: Option<bool>,

/// 
    #[serde(rename = "DisableAdvancedOptionsEditButton")]
    pub disable_advanced_options_edit_button: Option<bool>,

/// 
    #[serde(rename = "DisableDisconnectButton")]
    pub disable_disconnect_button: Option<bool>,

/// 
    #[serde(rename = "DisableIKEv2Fragmentation")]
    pub disable_ikev2_fragmentation: Option<bool>,

/// 
    #[serde(rename = "DnsSuffix")]
    pub dns_suffix: Option<String>,

/// 
    #[serde(rename = "EdpModeId")]
    pub edp_mode_id: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "IPv4InterfaceMetric")]
    pub ipv4_interface_metric: Option<i32>,

/// 
    #[serde(rename = "IPv6InterfaceMetric")]
    pub ipv6_interface_metric: Option<i32>,

/// 
    #[serde(rename = "NetworkOutageTime")]
    pub network_outage_time: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PrivateNetwork")]
    pub private_network: Option<bool>,

/// 
    #[serde(rename = "ProfileXML")]
    pub profile_xml: Option<String>,

/// 
    #[serde(rename = "RegisterDNS")]
    pub register_dns: Option<bool>,

/// 
    #[serde(rename = "RememberCredentials")]
    pub remember_credentials: Option<bool>,

/// 
    #[serde(rename = "TrustedNetworkDetection")]
    pub trusted_network_detection: Option<String>,

/// 
    #[serde(rename = "UseRasCredentials")]
    pub use_ras_credentials: Option<bool>,
}

impl MDM_VPNv2_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            always_on: None,
            always_on_active: None,
            by_pass_for_local: None,
            data_encryption: None,
            device_tunnel: None,
            disable_advanced_options_edit_button: None,
            disable_disconnect_button: None,
            disable_ikev2_fragmentation: None,
            dns_suffix: None,
            edp_mode_id: None,
            instance_id: None,
            ipv4_interface_metric: None,
            ipv6_interface_metric: None,
            network_outage_time: None,
            parent_id: None,
            private_network: None,
            profile_xml: None,
            register_dns: None,
            remember_credentials: None,
            trusted_network_detection: None,
            use_ras_credentials: None,
        }
    }


    /// Sets the value of AlwaysOn
    pub fn set_always_on(&mut self, value: bool) {
        self.always_on = Some(value);
    }

    /// Gets the value of AlwaysOn
    pub fn get_always_on(&self) -> Option<&bool> {
        self.always_on.as_ref()
    }

    /// Sets the value of AlwaysOnActive
    pub fn set_always_on_active(&mut self, value: bool) {
        self.always_on_active = Some(value);
    }

    /// Gets the value of AlwaysOnActive
    pub fn get_always_on_active(&self) -> Option<&bool> {
        self.always_on_active.as_ref()
    }

    /// Sets the value of ByPassForLocal
    pub fn set_by_pass_for_local(&mut self, value: bool) {
        self.by_pass_for_local = Some(value);
    }

    /// Gets the value of ByPassForLocal
    pub fn get_by_pass_for_local(&self) -> Option<&bool> {
        self.by_pass_for_local.as_ref()
    }

    /// Sets the value of DataEncryption
    pub fn set_data_encryption(&mut self, value: String) {
        self.data_encryption = Some(value);
    }

    /// Gets the value of DataEncryption
    pub fn get_data_encryption(&self) -> Option<&String> {
        self.data_encryption.as_ref()
    }

    /// Sets the value of DeviceTunnel
    pub fn set_device_tunnel(&mut self, value: bool) {
        self.device_tunnel = Some(value);
    }

    /// Gets the value of DeviceTunnel
    pub fn get_device_tunnel(&self) -> Option<&bool> {
        self.device_tunnel.as_ref()
    }

    /// Sets the value of DisableAdvancedOptionsEditButton
    pub fn set_disable_advanced_options_edit_button(&mut self, value: bool) {
        self.disable_advanced_options_edit_button = Some(value);
    }

    /// Gets the value of DisableAdvancedOptionsEditButton
    pub fn get_disable_advanced_options_edit_button(&self) -> Option<&bool> {
        self.disable_advanced_options_edit_button.as_ref()
    }

    /// Sets the value of DisableDisconnectButton
    pub fn set_disable_disconnect_button(&mut self, value: bool) {
        self.disable_disconnect_button = Some(value);
    }

    /// Gets the value of DisableDisconnectButton
    pub fn get_disable_disconnect_button(&self) -> Option<&bool> {
        self.disable_disconnect_button.as_ref()
    }

    /// Sets the value of DisableIKEv2Fragmentation
    pub fn set_disable_ikev2_fragmentation(&mut self, value: bool) {
        self.disable_ikev2_fragmentation = Some(value);
    }

    /// Gets the value of DisableIKEv2Fragmentation
    pub fn get_disable_ikev2_fragmentation(&self) -> Option<&bool> {
        self.disable_ikev2_fragmentation.as_ref()
    }

    /// Sets the value of DnsSuffix
    pub fn set_dns_suffix(&mut self, value: String) {
        self.dns_suffix = Some(value);
    }

    /// Gets the value of DnsSuffix
    pub fn get_dns_suffix(&self) -> Option<&String> {
        self.dns_suffix.as_ref()
    }

    /// Sets the value of EdpModeId
    pub fn set_edp_mode_id(&mut self, value: String) {
        self.edp_mode_id = Some(value);
    }

    /// Gets the value of EdpModeId
    pub fn get_edp_mode_id(&self) -> Option<&String> {
        self.edp_mode_id.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of IPv4InterfaceMetric
    pub fn set_ipv4_interface_metric(&mut self, value: i32) {
        self.ipv4_interface_metric = Some(value);
    }

    /// Gets the value of IPv4InterfaceMetric
    pub fn get_ipv4_interface_metric(&self) -> Option<&i32> {
        self.ipv4_interface_metric.as_ref()
    }

    /// Sets the value of IPv6InterfaceMetric
    pub fn set_ipv6_interface_metric(&mut self, value: i32) {
        self.ipv6_interface_metric = Some(value);
    }

    /// Gets the value of IPv6InterfaceMetric
    pub fn get_ipv6_interface_metric(&self) -> Option<&i32> {
        self.ipv6_interface_metric.as_ref()
    }

    /// Sets the value of NetworkOutageTime
    pub fn set_network_outage_time(&mut self, value: i32) {
        self.network_outage_time = Some(value);
    }

    /// Gets the value of NetworkOutageTime
    pub fn get_network_outage_time(&self) -> Option<&i32> {
        self.network_outage_time.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PrivateNetwork
    pub fn set_private_network(&mut self, value: bool) {
        self.private_network = Some(value);
    }

    /// Gets the value of PrivateNetwork
    pub fn get_private_network(&self) -> Option<&bool> {
        self.private_network.as_ref()
    }

    /// Sets the value of ProfileXML
    pub fn set_profile_xml(&mut self, value: String) {
        self.profile_xml = Some(value);
    }

    /// Gets the value of ProfileXML
    pub fn get_profile_xml(&self) -> Option<&String> {
        self.profile_xml.as_ref()
    }

    /// Sets the value of RegisterDNS
    pub fn set_register_dns(&mut self, value: bool) {
        self.register_dns = Some(value);
    }

    /// Gets the value of RegisterDNS
    pub fn get_register_dns(&self) -> Option<&bool> {
        self.register_dns.as_ref()
    }

    /// Sets the value of RememberCredentials
    pub fn set_remember_credentials(&mut self, value: bool) {
        self.remember_credentials = Some(value);
    }

    /// Gets the value of RememberCredentials
    pub fn get_remember_credentials(&self) -> Option<&bool> {
        self.remember_credentials.as_ref()
    }

    /// Sets the value of TrustedNetworkDetection
    pub fn set_trusted_network_detection(&mut self, value: String) {
        self.trusted_network_detection = Some(value);
    }

    /// Gets the value of TrustedNetworkDetection
    pub fn get_trusted_network_detection(&self) -> Option<&String> {
        self.trusted_network_detection.as_ref()
    }

    /// Sets the value of UseRasCredentials
    pub fn set_use_ras_credentials(&mut self, value: bool) {
        self.use_ras_credentials = Some(value);
    }

    /// Gets the value of UseRasCredentials
    pub fn get_use_ras_credentials(&self) -> Option<&bool> {
        self.use_ras_credentials.as_ref()
    }
}

