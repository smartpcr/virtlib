// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetFirewallProfile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetFirewallProfile {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "AllowInboundRules")]
    pub allow_inbound_rules: Option<u16>,

/// 
    #[serde(rename = "AllowLocalFirewallRules")]
    pub allow_local_firewall_rules: Option<u16>,

/// 
    #[serde(rename = "AllowLocalIPsecRules")]
    pub allow_local_ipsec_rules: Option<u16>,

/// 
    #[serde(rename = "AllowUnicastResponseToMulticast")]
    pub allow_unicast_response_to_multicast: Option<u16>,

/// 
    #[serde(rename = "AllowUserApps")]
    pub allow_user_apps: Option<u16>,

/// 
    #[serde(rename = "AllowUserPorts")]
    pub allow_user_ports: Option<u16>,

/// 
    #[serde(rename = "DefaultInboundAction")]
    pub default_inbound_action: Option<u16>,

/// 
    #[serde(rename = "DefaultOutboundAction")]
    pub default_outbound_action: Option<u16>,

/// 
    #[serde(rename = "DisabledInterfaceAliases")]
    pub disabled_interface_aliases: Vec<String>,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<u16>,

/// 
    #[serde(rename = "EnableStealthModeForIPsec")]
    pub enable_stealth_mode_for_ipsec: Option<u16>,

/// 
    #[serde(rename = "LogAllowed")]
    pub log_allowed: Option<u16>,

/// 
    #[serde(rename = "LogBlocked")]
    pub log_blocked: Option<u16>,

/// 
    #[serde(rename = "LogFileName")]
    pub log_file_name: Option<String>,

/// 
    #[serde(rename = "LogIgnored")]
    pub log_ignored: Option<u16>,

/// 
    #[serde(rename = "LogMaxSizeKilobytes")]
    pub log_max_size_kilobytes: Option<u64>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "NotifyOnListen")]
    pub notify_on_listen: Option<u16>,
}

impl MSFT_NetFirewallProfile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            allow_inbound_rules: None,
            allow_local_firewall_rules: None,
            allow_local_ipsec_rules: None,
            allow_unicast_response_to_multicast: None,
            allow_user_apps: None,
            allow_user_ports: None,
            default_inbound_action: None,
            default_outbound_action: None,
            disabled_interface_aliases: Vec::new(),
            enabled: None,
            enable_stealth_mode_for_ipsec: None,
            log_allowed: None,
            log_blocked: None,
            log_file_name: None,
            log_ignored: None,
            log_max_size_kilobytes: None,
            name: None,
            notify_on_listen: None,
        }
    }


    /// Sets the value of AllowInboundRules
    pub fn set_allow_inbound_rules(&mut self, value: u16) {
        self.allow_inbound_rules = Some(value);
    }

    /// Gets the value of AllowInboundRules
    pub fn get_allow_inbound_rules(&self) -> Option<&u16> {
        self.allow_inbound_rules.as_ref()
    }

    /// Sets the value of AllowLocalFirewallRules
    pub fn set_allow_local_firewall_rules(&mut self, value: u16) {
        self.allow_local_firewall_rules = Some(value);
    }

    /// Gets the value of AllowLocalFirewallRules
    pub fn get_allow_local_firewall_rules(&self) -> Option<&u16> {
        self.allow_local_firewall_rules.as_ref()
    }

    /// Sets the value of AllowLocalIPsecRules
    pub fn set_allow_local_ipsec_rules(&mut self, value: u16) {
        self.allow_local_ipsec_rules = Some(value);
    }

    /// Gets the value of AllowLocalIPsecRules
    pub fn get_allow_local_ipsec_rules(&self) -> Option<&u16> {
        self.allow_local_ipsec_rules.as_ref()
    }

    /// Sets the value of AllowUnicastResponseToMulticast
    pub fn set_allow_unicast_response_to_multicast(&mut self, value: u16) {
        self.allow_unicast_response_to_multicast = Some(value);
    }

    /// Gets the value of AllowUnicastResponseToMulticast
    pub fn get_allow_unicast_response_to_multicast(&self) -> Option<&u16> {
        self.allow_unicast_response_to_multicast.as_ref()
    }

    /// Sets the value of AllowUserApps
    pub fn set_allow_user_apps(&mut self, value: u16) {
        self.allow_user_apps = Some(value);
    }

    /// Gets the value of AllowUserApps
    pub fn get_allow_user_apps(&self) -> Option<&u16> {
        self.allow_user_apps.as_ref()
    }

    /// Sets the value of AllowUserPorts
    pub fn set_allow_user_ports(&mut self, value: u16) {
        self.allow_user_ports = Some(value);
    }

    /// Gets the value of AllowUserPorts
    pub fn get_allow_user_ports(&self) -> Option<&u16> {
        self.allow_user_ports.as_ref()
    }

    /// Sets the value of DefaultInboundAction
    pub fn set_default_inbound_action(&mut self, value: u16) {
        self.default_inbound_action = Some(value);
    }

    /// Gets the value of DefaultInboundAction
    pub fn get_default_inbound_action(&self) -> Option<&u16> {
        self.default_inbound_action.as_ref()
    }

    /// Sets the value of DefaultOutboundAction
    pub fn set_default_outbound_action(&mut self, value: u16) {
        self.default_outbound_action = Some(value);
    }

    /// Gets the value of DefaultOutboundAction
    pub fn get_default_outbound_action(&self) -> Option<&u16> {
        self.default_outbound_action.as_ref()
    }

    /// Sets the value of DisabledInterfaceAliases
    pub fn set_disabled_interface_aliases(&mut self, value: Vec<String>) {
        self.disabled_interface_aliases = value;
    }

    /// Gets the value of DisabledInterfaceAliases
    pub fn get_disabled_interface_aliases(&self) -> &Vec<String> {
        &self.disabled_interface_aliases
    }

    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: u16) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&u16> {
        self.enabled.as_ref()
    }

    /// Sets the value of EnableStealthModeForIPsec
    pub fn set_enable_stealth_mode_for_ipsec(&mut self, value: u16) {
        self.enable_stealth_mode_for_ipsec = Some(value);
    }

    /// Gets the value of EnableStealthModeForIPsec
    pub fn get_enable_stealth_mode_for_ipsec(&self) -> Option<&u16> {
        self.enable_stealth_mode_for_ipsec.as_ref()
    }

    /// Sets the value of LogAllowed
    pub fn set_log_allowed(&mut self, value: u16) {
        self.log_allowed = Some(value);
    }

    /// Gets the value of LogAllowed
    pub fn get_log_allowed(&self) -> Option<&u16> {
        self.log_allowed.as_ref()
    }

    /// Sets the value of LogBlocked
    pub fn set_log_blocked(&mut self, value: u16) {
        self.log_blocked = Some(value);
    }

    /// Gets the value of LogBlocked
    pub fn get_log_blocked(&self) -> Option<&u16> {
        self.log_blocked.as_ref()
    }

    /// Sets the value of LogFileName
    pub fn set_log_file_name(&mut self, value: String) {
        self.log_file_name = Some(value);
    }

    /// Gets the value of LogFileName
    pub fn get_log_file_name(&self) -> Option<&String> {
        self.log_file_name.as_ref()
    }

    /// Sets the value of LogIgnored
    pub fn set_log_ignored(&mut self, value: u16) {
        self.log_ignored = Some(value);
    }

    /// Gets the value of LogIgnored
    pub fn get_log_ignored(&self) -> Option<&u16> {
        self.log_ignored.as_ref()
    }

    /// Sets the value of LogMaxSizeKilobytes
    pub fn set_log_max_size_kilobytes(&mut self, value: u64) {
        self.log_max_size_kilobytes = Some(value);
    }

    /// Gets the value of LogMaxSizeKilobytes
    pub fn get_log_max_size_kilobytes(&self) -> Option<&u64> {
        self.log_max_size_kilobytes.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of NotifyOnListen
    pub fn set_notify_on_listen(&mut self, value: u16) {
        self.notify_on_listen = Some(value);
    }

    /// Gets the value of NotifyOnListen
    pub fn get_notify_on_listen(&self) -> Option<&u16> {
        self.notify_on_listen.as_ref()
    }
}

