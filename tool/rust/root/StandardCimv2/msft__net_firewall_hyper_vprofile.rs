// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetFirewallHyperVProfile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetFirewallHyperVProfile {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "AllowLocalFirewallRules")]
    pub allow_local_firewall_rules: Option<u16>,

/// 
    #[serde(rename = "DefaultInboundAction")]
    pub default_inbound_action: Option<u16>,

/// 
    #[serde(rename = "DefaultOutboundAction")]
    pub default_outbound_action: Option<u16>,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<u16>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Profile")]
    pub profile: Option<u16>,
}

impl MSFT_NetFirewallHyperVProfile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            allow_local_firewall_rules: None,
            default_inbound_action: None,
            default_outbound_action: None,
            enabled: None,
            name: None,
            profile: None,
        }
    }


    /// Sets the value of AllowLocalFirewallRules
    pub fn set_allow_local_firewall_rules(&mut self, value: u16) {
        self.allow_local_firewall_rules = Some(value);
    }

    /// Gets the value of AllowLocalFirewallRules
    pub fn get_allow_local_firewall_rules(&self) -> Option<&u16> {
        self.allow_local_firewall_rules.as_ref()
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

    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: u16) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&u16> {
        self.enabled.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Profile
    pub fn set_profile(&mut self, value: u16) {
        self.profile = Some(value);
    }

    /// Gets the value of Profile
    pub fn get_profile(&self) -> Option<&u16> {
        self.profile.as_ref()
    }
}

