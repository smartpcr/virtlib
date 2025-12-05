// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetFirewallHyperVVMSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetFirewallHyperVVMSetting {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "AllowHostPolicyMerge")]
    pub allow_host_policy_merge: Option<u16>,

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
    #[serde(rename = "LoopbackEnabled")]
    pub loopback_enabled: Option<u16>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl MSFT_NetFirewallHyperVVMSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            allow_host_policy_merge: None,
            default_inbound_action: None,
            default_outbound_action: None,
            enabled: None,
            loopback_enabled: None,
            name: None,
        }
    }


    /// Sets the value of AllowHostPolicyMerge
    pub fn set_allow_host_policy_merge(&mut self, value: u16) {
        self.allow_host_policy_merge = Some(value);
    }

    /// Gets the value of AllowHostPolicyMerge
    pub fn get_allow_host_policy_merge(&self) -> Option<&u16> {
        self.allow_host_policy_merge.as_ref()
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

    /// Sets the value of LoopbackEnabled
    pub fn set_loopback_enabled(&mut self, value: u16) {
        self.loopback_enabled = Some(value);
    }

    /// Gets the value of LoopbackEnabled
    pub fn get_loopback_enabled(&self) -> Option<&u16> {
        self.loopback_enabled.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

