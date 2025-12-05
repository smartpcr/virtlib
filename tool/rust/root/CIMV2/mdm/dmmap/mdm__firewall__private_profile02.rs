// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Firewall_PrivateProfile02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Firewall_PrivateProfile02 {

/// 
    #[serde(rename = "AllowLocalIpsecPolicyMerge")]
    pub allow_local_ipsec_policy_merge: Option<bool>,

/// 
    #[serde(rename = "AllowLocalPolicyMerge")]
    pub allow_local_policy_merge: Option<bool>,

/// 
    #[serde(rename = "AuthAppsAllowUserPrefMerge")]
    pub auth_apps_allow_user_pref_merge: Option<bool>,

/// 
    #[serde(rename = "DefaultInboundAction")]
    pub default_inbound_action: Option<i32>,

/// 
    #[serde(rename = "DefaultOutboundAction")]
    pub default_outbound_action: Option<i32>,

/// 
    #[serde(rename = "DisableInboundNotifications")]
    pub disable_inbound_notifications: Option<bool>,

/// 
    #[serde(rename = "DisableStealthMode")]
    pub disable_stealth_mode: Option<bool>,

/// 
    #[serde(rename = "DisableStealthModeIpsecSecuredPacketExemption")]
    pub disable_stealth_mode_ipsec_secured_packet_exemption: Option<bool>,

/// 
    #[serde(rename = "DisableUnicastResponsesToMulticastBroadcast")]
    pub disable_unicast_responses_to_multicast_broadcast: Option<bool>,

/// 
    #[serde(rename = "EnableFirewall")]
    pub enable_firewall: Option<bool>,

/// 
    #[serde(rename = "EnableLogDroppedPackets")]
    pub enable_log_dropped_packets: Option<bool>,

/// 
    #[serde(rename = "EnableLogIgnoredRules")]
    pub enable_log_ignored_rules: Option<bool>,

/// 
    #[serde(rename = "EnableLogSuccessConnections")]
    pub enable_log_success_connections: Option<bool>,

/// 
    #[serde(rename = "GlobalPortsAllowUserPrefMerge")]
    pub global_ports_allow_user_pref_merge: Option<bool>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LogFilePath")]
    pub log_file_path: Option<String>,

/// 
    #[serde(rename = "LogMaxFileSize")]
    pub log_max_file_size: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "Shielded")]
    pub shielded: Option<bool>,
}

impl MDM_Firewall_PrivateProfile02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_local_ipsec_policy_merge: None,
            allow_local_policy_merge: None,
            auth_apps_allow_user_pref_merge: None,
            default_inbound_action: None,
            default_outbound_action: None,
            disable_inbound_notifications: None,
            disable_stealth_mode: None,
            disable_stealth_mode_ipsec_secured_packet_exemption: None,
            disable_unicast_responses_to_multicast_broadcast: None,
            enable_firewall: None,
            enable_log_dropped_packets: None,
            enable_log_ignored_rules: None,
            enable_log_success_connections: None,
            global_ports_allow_user_pref_merge: None,
            instance_id: None,
            log_file_path: None,
            log_max_file_size: None,
            parent_id: None,
            shielded: None,
        }
    }


    /// Sets the value of AllowLocalIpsecPolicyMerge
    pub fn set_allow_local_ipsec_policy_merge(&mut self, value: bool) {
        self.allow_local_ipsec_policy_merge = Some(value);
    }

    /// Gets the value of AllowLocalIpsecPolicyMerge
    pub fn get_allow_local_ipsec_policy_merge(&self) -> Option<&bool> {
        self.allow_local_ipsec_policy_merge.as_ref()
    }

    /// Sets the value of AllowLocalPolicyMerge
    pub fn set_allow_local_policy_merge(&mut self, value: bool) {
        self.allow_local_policy_merge = Some(value);
    }

    /// Gets the value of AllowLocalPolicyMerge
    pub fn get_allow_local_policy_merge(&self) -> Option<&bool> {
        self.allow_local_policy_merge.as_ref()
    }

    /// Sets the value of AuthAppsAllowUserPrefMerge
    pub fn set_auth_apps_allow_user_pref_merge(&mut self, value: bool) {
        self.auth_apps_allow_user_pref_merge = Some(value);
    }

    /// Gets the value of AuthAppsAllowUserPrefMerge
    pub fn get_auth_apps_allow_user_pref_merge(&self) -> Option<&bool> {
        self.auth_apps_allow_user_pref_merge.as_ref()
    }

    /// Sets the value of DefaultInboundAction
    pub fn set_default_inbound_action(&mut self, value: i32) {
        self.default_inbound_action = Some(value);
    }

    /// Gets the value of DefaultInboundAction
    pub fn get_default_inbound_action(&self) -> Option<&i32> {
        self.default_inbound_action.as_ref()
    }

    /// Sets the value of DefaultOutboundAction
    pub fn set_default_outbound_action(&mut self, value: i32) {
        self.default_outbound_action = Some(value);
    }

    /// Gets the value of DefaultOutboundAction
    pub fn get_default_outbound_action(&self) -> Option<&i32> {
        self.default_outbound_action.as_ref()
    }

    /// Sets the value of DisableInboundNotifications
    pub fn set_disable_inbound_notifications(&mut self, value: bool) {
        self.disable_inbound_notifications = Some(value);
    }

    /// Gets the value of DisableInboundNotifications
    pub fn get_disable_inbound_notifications(&self) -> Option<&bool> {
        self.disable_inbound_notifications.as_ref()
    }

    /// Sets the value of DisableStealthMode
    pub fn set_disable_stealth_mode(&mut self, value: bool) {
        self.disable_stealth_mode = Some(value);
    }

    /// Gets the value of DisableStealthMode
    pub fn get_disable_stealth_mode(&self) -> Option<&bool> {
        self.disable_stealth_mode.as_ref()
    }

    /// Sets the value of DisableStealthModeIpsecSecuredPacketExemption
    pub fn set_disable_stealth_mode_ipsec_secured_packet_exemption(&mut self, value: bool) {
        self.disable_stealth_mode_ipsec_secured_packet_exemption = Some(value);
    }

    /// Gets the value of DisableStealthModeIpsecSecuredPacketExemption
    pub fn get_disable_stealth_mode_ipsec_secured_packet_exemption(&self) -> Option<&bool> {
        self.disable_stealth_mode_ipsec_secured_packet_exemption.as_ref()
    }

    /// Sets the value of DisableUnicastResponsesToMulticastBroadcast
    pub fn set_disable_unicast_responses_to_multicast_broadcast(&mut self, value: bool) {
        self.disable_unicast_responses_to_multicast_broadcast = Some(value);
    }

    /// Gets the value of DisableUnicastResponsesToMulticastBroadcast
    pub fn get_disable_unicast_responses_to_multicast_broadcast(&self) -> Option<&bool> {
        self.disable_unicast_responses_to_multicast_broadcast.as_ref()
    }

    /// Sets the value of EnableFirewall
    pub fn set_enable_firewall(&mut self, value: bool) {
        self.enable_firewall = Some(value);
    }

    /// Gets the value of EnableFirewall
    pub fn get_enable_firewall(&self) -> Option<&bool> {
        self.enable_firewall.as_ref()
    }

    /// Sets the value of EnableLogDroppedPackets
    pub fn set_enable_log_dropped_packets(&mut self, value: bool) {
        self.enable_log_dropped_packets = Some(value);
    }

    /// Gets the value of EnableLogDroppedPackets
    pub fn get_enable_log_dropped_packets(&self) -> Option<&bool> {
        self.enable_log_dropped_packets.as_ref()
    }

    /// Sets the value of EnableLogIgnoredRules
    pub fn set_enable_log_ignored_rules(&mut self, value: bool) {
        self.enable_log_ignored_rules = Some(value);
    }

    /// Gets the value of EnableLogIgnoredRules
    pub fn get_enable_log_ignored_rules(&self) -> Option<&bool> {
        self.enable_log_ignored_rules.as_ref()
    }

    /// Sets the value of EnableLogSuccessConnections
    pub fn set_enable_log_success_connections(&mut self, value: bool) {
        self.enable_log_success_connections = Some(value);
    }

    /// Gets the value of EnableLogSuccessConnections
    pub fn get_enable_log_success_connections(&self) -> Option<&bool> {
        self.enable_log_success_connections.as_ref()
    }

    /// Sets the value of GlobalPortsAllowUserPrefMerge
    pub fn set_global_ports_allow_user_pref_merge(&mut self, value: bool) {
        self.global_ports_allow_user_pref_merge = Some(value);
    }

    /// Gets the value of GlobalPortsAllowUserPrefMerge
    pub fn get_global_ports_allow_user_pref_merge(&self) -> Option<&bool> {
        self.global_ports_allow_user_pref_merge.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of LogFilePath
    pub fn set_log_file_path(&mut self, value: String) {
        self.log_file_path = Some(value);
    }

    /// Gets the value of LogFilePath
    pub fn get_log_file_path(&self) -> Option<&String> {
        self.log_file_path.as_ref()
    }

    /// Sets the value of LogMaxFileSize
    pub fn set_log_max_file_size(&mut self, value: i32) {
        self.log_max_file_size = Some(value);
    }

    /// Gets the value of LogMaxFileSize
    pub fn get_log_max_file_size(&self) -> Option<&i32> {
        self.log_max_file_size.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of Shielded
    pub fn set_shielded(&mut self, value: bool) {
        self.shielded = Some(value);
    }

    /// Gets the value of Shielded
    pub fn get_shielded(&self) -> Option<&bool> {
        self.shielded.as_ref()
    }
}

