// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_PolicySet struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_PolicySet {
    #[serde(flatten)]
    pub base: CIM_Policy,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<u16>,

/// 
    #[serde(rename = "PolicyDecisionStrategy")]
    pub policy_decision_strategy: Option<u16>,

/// 
    #[serde(rename = "PolicyRoles")]
    pub policy_roles: Vec<String>,
}

impl CIM_PolicySet {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Policy::new(),
            enabled: None,
            policy_decision_strategy: None,
            policy_roles: Vec::new(),
        }
    }


    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: u16) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&u16> {
        self.enabled.as_ref()
    }

    /// Sets the value of PolicyDecisionStrategy
    pub fn set_policy_decision_strategy(&mut self, value: u16) {
        self.policy_decision_strategy = Some(value);
    }

    /// Gets the value of PolicyDecisionStrategy
    pub fn get_policy_decision_strategy(&self) -> Option<&u16> {
        self.policy_decision_strategy.as_ref()
    }

    /// Sets the value of PolicyRoles
    pub fn set_policy_roles(&mut self, value: Vec<String>) {
        self.policy_roles = value;
    }

    /// Gets the value of PolicyRoles
    pub fn get_policy_roles(&self) -> &Vec<String> {
        &self.policy_roles
    }
}

