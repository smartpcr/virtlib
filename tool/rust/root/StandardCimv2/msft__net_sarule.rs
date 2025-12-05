// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetSARule struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetSARule {
    #[serde(flatten)]
    pub base: CIM_SARule,

/// 
    #[serde(rename = "DisplayGroup")]
    pub display_group: Option<String>,

/// 
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "EnforcementStatus")]
    pub enforcement_status: Vec<u16>,

/// 
    #[serde(rename = "MainModeCryptoSet")]
    pub main_mode_crypto_set: Option<String>,

/// 
    #[serde(rename = "Phase1AuthSet")]
    pub phase1_auth_set: Option<String>,

/// 
    #[serde(rename = "Phase2AuthSet")]
    pub phase2_auth_set: Option<String>,

/// 
    #[serde(rename = "Platforms")]
    pub platforms: Vec<String>,

/// 
    #[serde(rename = "PolicyStoreSource")]
    pub policy_store_source: Option<String>,

/// 
    #[serde(rename = "PolicyStoreSourceType")]
    pub policy_store_source_type: Option<u16>,

/// 
    #[serde(rename = "PrimaryStatus")]
    pub primary_status: Option<u16>,

/// 
    #[serde(rename = "Profiles")]
    pub profiles: Option<u16>,

/// 
    #[serde(rename = "QuickModeCryptoSet")]
    pub quick_mode_crypto_set: Option<String>,

/// 
    #[serde(rename = "RuleGroup")]
    pub rule_group: Option<String>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<String>,

/// 
    #[serde(rename = "StatusCode")]
    pub status_code: Option<u32>,
}

impl MSFT_NetSARule {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SARule::new(),
            display_group: None,
            display_name: None,
            enforcement_status: Vec::new(),
            main_mode_crypto_set: None,
            phase1_auth_set: None,
            phase2_auth_set: None,
            platforms: Vec::new(),
            policy_store_source: None,
            policy_store_source_type: None,
            primary_status: None,
            profiles: None,
            quick_mode_crypto_set: None,
            rule_group: None,
            status: None,
            status_code: None,
        }
    }


    /// Sets the value of DisplayGroup
    pub fn set_display_group(&mut self, value: String) {
        self.display_group = Some(value);
    }

    /// Gets the value of DisplayGroup
    pub fn get_display_group(&self) -> Option<&String> {
        self.display_group.as_ref()
    }

    /// Sets the value of DisplayName
    pub fn set_display_name(&mut self, value: String) {
        self.display_name = Some(value);
    }

    /// Gets the value of DisplayName
    pub fn get_display_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }

    /// Sets the value of EnforcementStatus
    pub fn set_enforcement_status(&mut self, value: Vec<u16>) {
        self.enforcement_status = value;
    }

    /// Gets the value of EnforcementStatus
    pub fn get_enforcement_status(&self) -> &Vec<u16> {
        &self.enforcement_status
    }

    /// Sets the value of MainModeCryptoSet
    pub fn set_main_mode_crypto_set(&mut self, value: String) {
        self.main_mode_crypto_set = Some(value);
    }

    /// Gets the value of MainModeCryptoSet
    pub fn get_main_mode_crypto_set(&self) -> Option<&String> {
        self.main_mode_crypto_set.as_ref()
    }

    /// Sets the value of Phase1AuthSet
    pub fn set_phase1_auth_set(&mut self, value: String) {
        self.phase1_auth_set = Some(value);
    }

    /// Gets the value of Phase1AuthSet
    pub fn get_phase1_auth_set(&self) -> Option<&String> {
        self.phase1_auth_set.as_ref()
    }

    /// Sets the value of Phase2AuthSet
    pub fn set_phase2_auth_set(&mut self, value: String) {
        self.phase2_auth_set = Some(value);
    }

    /// Gets the value of Phase2AuthSet
    pub fn get_phase2_auth_set(&self) -> Option<&String> {
        self.phase2_auth_set.as_ref()
    }

    /// Sets the value of Platforms
    pub fn set_platforms(&mut self, value: Vec<String>) {
        self.platforms = value;
    }

    /// Gets the value of Platforms
    pub fn get_platforms(&self) -> &Vec<String> {
        &self.platforms
    }

    /// Sets the value of PolicyStoreSource
    pub fn set_policy_store_source(&mut self, value: String) {
        self.policy_store_source = Some(value);
    }

    /// Gets the value of PolicyStoreSource
    pub fn get_policy_store_source(&self) -> Option<&String> {
        self.policy_store_source.as_ref()
    }

    /// Sets the value of PolicyStoreSourceType
    pub fn set_policy_store_source_type(&mut self, value: u16) {
        self.policy_store_source_type = Some(value);
    }

    /// Gets the value of PolicyStoreSourceType
    pub fn get_policy_store_source_type(&self) -> Option<&u16> {
        self.policy_store_source_type.as_ref()
    }

    /// Sets the value of PrimaryStatus
    pub fn set_primary_status(&mut self, value: u16) {
        self.primary_status = Some(value);
    }

    /// Gets the value of PrimaryStatus
    pub fn get_primary_status(&self) -> Option<&u16> {
        self.primary_status.as_ref()
    }

    /// Sets the value of Profiles
    pub fn set_profiles(&mut self, value: u16) {
        self.profiles = Some(value);
    }

    /// Gets the value of Profiles
    pub fn get_profiles(&self) -> Option<&u16> {
        self.profiles.as_ref()
    }

    /// Sets the value of QuickModeCryptoSet
    pub fn set_quick_mode_crypto_set(&mut self, value: String) {
        self.quick_mode_crypto_set = Some(value);
    }

    /// Gets the value of QuickModeCryptoSet
    pub fn get_quick_mode_crypto_set(&self) -> Option<&String> {
        self.quick_mode_crypto_set.as_ref()
    }

    /// Sets the value of RuleGroup
    pub fn set_rule_group(&mut self, value: String) {
        self.rule_group = Some(value);
    }

    /// Gets the value of RuleGroup
    pub fn get_rule_group(&self) -> Option<&String> {
        self.rule_group.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: String) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&String> {
        self.status.as_ref()
    }

    /// Sets the value of StatusCode
    pub fn set_status_code(&mut self, value: u32) {
        self.status_code = Some(value);
    }

    /// Gets the value of StatusCode
    pub fn get_status_code(&self) -> Option<&u32> {
        self.status_code.as_ref()
    }
}

