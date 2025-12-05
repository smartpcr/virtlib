// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_PolicyAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_PolicyAction {
    #[serde(flatten)]
    pub base: CIM_Policy,

/// 
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// 
    #[serde(rename = "DoActionLogging")]
    pub do_action_logging: Option<bool>,

/// 
    #[serde(rename = "PolicyActionName")]
    pub policy_action_name: Option<String>,

/// 
    #[serde(rename = "PolicyRuleCreationClassName")]
    pub policy_rule_creation_class_name: Option<String>,

/// 
    #[serde(rename = "PolicyRuleName")]
    pub policy_rule_name: Option<String>,

/// 
    #[serde(rename = "SystemCreationClassName")]
    pub system_creation_class_name: Option<String>,

/// 
    #[serde(rename = "SystemName")]
    pub system_name: Option<String>,
}

impl CIM_PolicyAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Policy::new(),
            creation_class_name: None,
            do_action_logging: None,
            policy_action_name: None,
            policy_rule_creation_class_name: None,
            policy_rule_name: None,
            system_creation_class_name: None,
            system_name: None,
        }
    }


    /// Sets the value of CreationClassName
    pub fn set_creation_class_name(&mut self, value: String) {
        self.creation_class_name = Some(value);
    }

    /// Gets the value of CreationClassName
    pub fn get_creation_class_name(&self) -> Option<&String> {
        self.creation_class_name.as_ref()
    }

    /// Sets the value of DoActionLogging
    pub fn set_do_action_logging(&mut self, value: bool) {
        self.do_action_logging = Some(value);
    }

    /// Gets the value of DoActionLogging
    pub fn get_do_action_logging(&self) -> Option<&bool> {
        self.do_action_logging.as_ref()
    }

    /// Sets the value of PolicyActionName
    pub fn set_policy_action_name(&mut self, value: String) {
        self.policy_action_name = Some(value);
    }

    /// Gets the value of PolicyActionName
    pub fn get_policy_action_name(&self) -> Option<&String> {
        self.policy_action_name.as_ref()
    }

    /// Sets the value of PolicyRuleCreationClassName
    pub fn set_policy_rule_creation_class_name(&mut self, value: String) {
        self.policy_rule_creation_class_name = Some(value);
    }

    /// Gets the value of PolicyRuleCreationClassName
    pub fn get_policy_rule_creation_class_name(&self) -> Option<&String> {
        self.policy_rule_creation_class_name.as_ref()
    }

    /// Sets the value of PolicyRuleName
    pub fn set_policy_rule_name(&mut self, value: String) {
        self.policy_rule_name = Some(value);
    }

    /// Gets the value of PolicyRuleName
    pub fn get_policy_rule_name(&self) -> Option<&String> {
        self.policy_rule_name.as_ref()
    }

    /// Sets the value of SystemCreationClassName
    pub fn set_system_creation_class_name(&mut self, value: String) {
        self.system_creation_class_name = Some(value);
    }

    /// Gets the value of SystemCreationClassName
    pub fn get_system_creation_class_name(&self) -> Option<&String> {
        self.system_creation_class_name.as_ref()
    }

    /// Sets the value of SystemName
    pub fn set_system_name(&mut self, value: String) {
        self.system_name = Some(value);
    }

    /// Gets the value of SystemName
    pub fn get_system_name(&self) -> Option<&String> {
        self.system_name.as_ref()
    }
}

