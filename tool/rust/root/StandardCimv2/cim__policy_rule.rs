// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_PolicyRule struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_PolicyRule {
    #[serde(flatten)]
    pub base: CIM_PolicySet,

/// 
    #[serde(rename = "ConditionListType")]
    pub condition_list_type: Option<u16>,

/// 
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// 
    #[serde(rename = "ExecutionStrategy")]
    pub execution_strategy: Option<u16>,

/// 
    #[serde(rename = "Mandatory")]
    pub mandatory: Option<bool>,

/// 
    #[serde(rename = "PolicyRuleName")]
    pub policy_rule_name: Option<String>,

/// 
    #[serde(rename = "Priority")]
    pub priority: Option<u16>,

/// 
    #[serde(rename = "RuleUsage")]
    pub rule_usage: Option<String>,

/// 
    #[serde(rename = "SequencedActions")]
    pub sequenced_actions: Option<u16>,

/// 
    #[serde(rename = "SystemCreationClassName")]
    pub system_creation_class_name: Option<String>,

/// 
    #[serde(rename = "SystemName")]
    pub system_name: Option<String>,
}

impl CIM_PolicyRule {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PolicySet::new(),
            condition_list_type: None,
            creation_class_name: None,
            execution_strategy: None,
            mandatory: None,
            policy_rule_name: None,
            priority: None,
            rule_usage: None,
            sequenced_actions: None,
            system_creation_class_name: None,
            system_name: None,
        }
    }


    /// Sets the value of ConditionListType
    pub fn set_condition_list_type(&mut self, value: u16) {
        self.condition_list_type = Some(value);
    }

    /// Gets the value of ConditionListType
    pub fn get_condition_list_type(&self) -> Option<&u16> {
        self.condition_list_type.as_ref()
    }

    /// Sets the value of CreationClassName
    pub fn set_creation_class_name(&mut self, value: String) {
        self.creation_class_name = Some(value);
    }

    /// Gets the value of CreationClassName
    pub fn get_creation_class_name(&self) -> Option<&String> {
        self.creation_class_name.as_ref()
    }

    /// Sets the value of ExecutionStrategy
    pub fn set_execution_strategy(&mut self, value: u16) {
        self.execution_strategy = Some(value);
    }

    /// Gets the value of ExecutionStrategy
    pub fn get_execution_strategy(&self) -> Option<&u16> {
        self.execution_strategy.as_ref()
    }

    /// Sets the value of Mandatory
    pub fn set_mandatory(&mut self, value: bool) {
        self.mandatory = Some(value);
    }

    /// Gets the value of Mandatory
    pub fn get_mandatory(&self) -> Option<&bool> {
        self.mandatory.as_ref()
    }

    /// Sets the value of PolicyRuleName
    pub fn set_policy_rule_name(&mut self, value: String) {
        self.policy_rule_name = Some(value);
    }

    /// Gets the value of PolicyRuleName
    pub fn get_policy_rule_name(&self) -> Option<&String> {
        self.policy_rule_name.as_ref()
    }

    /// Sets the value of Priority
    pub fn set_priority(&mut self, value: u16) {
        self.priority = Some(value);
    }

    /// Gets the value of Priority
    pub fn get_priority(&self) -> Option<&u16> {
        self.priority.as_ref()
    }

    /// Sets the value of RuleUsage
    pub fn set_rule_usage(&mut self, value: String) {
        self.rule_usage = Some(value);
    }

    /// Gets the value of RuleUsage
    pub fn get_rule_usage(&self) -> Option<&String> {
        self.rule_usage.as_ref()
    }

    /// Sets the value of SequencedActions
    pub fn set_sequenced_actions(&mut self, value: u16) {
        self.sequenced_actions = Some(value);
    }

    /// Gets the value of SequencedActions
    pub fn get_sequenced_actions(&self) -> Option<&u16> {
        self.sequenced_actions.as_ref()
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

