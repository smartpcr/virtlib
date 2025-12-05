// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetSecDeltaCollection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetSecDeltaCollection {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "Action")]
    pub action: Option<u16>,

/// 
    #[serde(rename = "EndpointType")]
    pub endpoint_type: Option<u16>,

/// 
    #[serde(rename = "IPsecRuleDisplayName")]
    pub ipsec_rule_display_name: Option<String>,

/// 
    #[serde(rename = "IPsecRuleName")]
    pub ipsec_rule_name: Option<String>,

/// 
    #[serde(rename = "IPv4Addresses")]
    pub ipv4_addresses: Vec<String>,

/// 
    #[serde(rename = "IPv6Addresses")]
    pub ipv6_addresses: Vec<String>,

/// 
    #[serde(rename = "NameResolutionFailures")]
    pub name_resolution_failures: Vec<String>,

/// 
    #[serde(rename = "PolicyStore")]
    pub policy_store: Option<String>,
}

impl MSFT_NetSecDeltaCollection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            action: None,
            endpoint_type: None,
            ipsec_rule_display_name: None,
            ipsec_rule_name: None,
            ipv4_addresses: Vec::new(),
            ipv6_addresses: Vec::new(),
            name_resolution_failures: Vec::new(),
            policy_store: None,
        }
    }


    /// Sets the value of Action
    pub fn set_action(&mut self, value: u16) {
        self.action = Some(value);
    }

    /// Gets the value of Action
    pub fn get_action(&self) -> Option<&u16> {
        self.action.as_ref()
    }

    /// Sets the value of EndpointType
    pub fn set_endpoint_type(&mut self, value: u16) {
        self.endpoint_type = Some(value);
    }

    /// Gets the value of EndpointType
    pub fn get_endpoint_type(&self) -> Option<&u16> {
        self.endpoint_type.as_ref()
    }

    /// Sets the value of IPsecRuleDisplayName
    pub fn set_ipsec_rule_display_name(&mut self, value: String) {
        self.ipsec_rule_display_name = Some(value);
    }

    /// Gets the value of IPsecRuleDisplayName
    pub fn get_ipsec_rule_display_name(&self) -> Option<&String> {
        self.ipsec_rule_display_name.as_ref()
    }

    /// Sets the value of IPsecRuleName
    pub fn set_ipsec_rule_name(&mut self, value: String) {
        self.ipsec_rule_name = Some(value);
    }

    /// Gets the value of IPsecRuleName
    pub fn get_ipsec_rule_name(&self) -> Option<&String> {
        self.ipsec_rule_name.as_ref()
    }

    /// Sets the value of IPv4Addresses
    pub fn set_ipv4_addresses(&mut self, value: Vec<String>) {
        self.ipv4_addresses = value;
    }

    /// Gets the value of IPv4Addresses
    pub fn get_ipv4_addresses(&self) -> &Vec<String> {
        &self.ipv4_addresses
    }

    /// Sets the value of IPv6Addresses
    pub fn set_ipv6_addresses(&mut self, value: Vec<String>) {
        self.ipv6_addresses = value;
    }

    /// Gets the value of IPv6Addresses
    pub fn get_ipv6_addresses(&self) -> &Vec<String> {
        &self.ipv6_addresses
    }

    /// Sets the value of NameResolutionFailures
    pub fn set_name_resolution_failures(&mut self, value: Vec<String>) {
        self.name_resolution_failures = value;
    }

    /// Gets the value of NameResolutionFailures
    pub fn get_name_resolution_failures(&self) -> &Vec<String> {
        &self.name_resolution_failures
    }

    /// Sets the value of PolicyStore
    pub fn set_policy_store(&mut self, value: String) {
        self.policy_store = Some(value);
    }

    /// Gets the value of PolicyStore
    pub fn get_policy_store(&self) -> Option<&String> {
        self.policy_store.as_ref()
    }
}

