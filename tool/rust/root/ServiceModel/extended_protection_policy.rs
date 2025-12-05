// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ExtendedProtectionPolicy struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExtendedProtectionPolicy {

/// A channel binding token used when authenticating clients.
    #[serde(rename = "CustomChannelBinding")]
    pub custom_channel_binding: Option<String>,

/// A list of service principal names accepted by the service.
    #[serde(rename = "CustomServiceNames")]
    pub custom_service_names: Vec<String>,

/// A value that specifies when ExtendedProtection should be enforced.
    #[serde(rename = "PolicyEnforcement")]
    pub policy_enforcement: Option<String>,

/// A value that specifies the protection scenario being enforced by the policy.
    #[serde(rename = "ProtectionScenario")]
    pub protection_scenario: Option<String>,
}

impl ExtendedProtectionPolicy {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            custom_channel_binding: None,
            custom_service_names: Vec::new(),
            policy_enforcement: None,
            protection_scenario: None,
        }
    }


    /// Sets the value of CustomChannelBinding
    pub fn set_custom_channel_binding(&mut self, value: String) {
        self.custom_channel_binding = Some(value);
    }

    /// Gets the value of CustomChannelBinding
    pub fn get_custom_channel_binding(&self) -> Option<&String> {
        self.custom_channel_binding.as_ref()
    }

    /// Sets the value of CustomServiceNames
    pub fn set_custom_service_names(&mut self, value: Vec<String>) {
        self.custom_service_names = value;
    }

    /// Gets the value of CustomServiceNames
    pub fn get_custom_service_names(&self) -> &Vec<String> {
        &self.custom_service_names
    }

    /// Sets the value of PolicyEnforcement
    pub fn set_policy_enforcement(&mut self, value: String) {
        self.policy_enforcement = Some(value);
    }

    /// Gets the value of PolicyEnforcement
    pub fn get_policy_enforcement(&self) -> Option<&String> {
        self.policy_enforcement.as_ref()
    }

    /// Sets the value of ProtectionScenario
    pub fn set_protection_scenario(&mut self, value: String) {
        self.protection_scenario = Some(value);
    }

    /// Gets the value of ProtectionScenario
    pub fn get_protection_scenario(&self) -> Option<&String> {
        self.protection_scenario.as_ref()
    }
}

