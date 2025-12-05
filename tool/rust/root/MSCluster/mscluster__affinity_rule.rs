// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_AffinityRule struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_AffinityRule {

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<u32>,

/// 
    #[serde(rename = "Groups")]
    pub groups: Vec<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "RuleType")]
    pub rule_type: Option<u32>,

/// 
    #[serde(rename = "SoftAntiAffinity")]
    pub soft_anti_affinity: Option<u32>,
}

impl MSCluster_AffinityRule {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            enabled: None,
            groups: Vec::new(),
            name: None,
            rule_type: None,
            soft_anti_affinity: None,
        }
    }


    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: u32) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&u32> {
        self.enabled.as_ref()
    }

    /// Sets the value of Groups
    pub fn set_groups(&mut self, value: Vec<String>) {
        self.groups = value;
    }

    /// Gets the value of Groups
    pub fn get_groups(&self) -> &Vec<String> {
        &self.groups
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of RuleType
    pub fn set_rule_type(&mut self, value: u32) {
        self.rule_type = Some(value);
    }

    /// Gets the value of RuleType
    pub fn get_rule_type(&self) -> Option<&u32> {
        self.rule_type.as_ref()
    }

    /// Sets the value of SoftAntiAffinity
    pub fn set_soft_anti_affinity(&mut self, value: u32) {
        self.soft_anti_affinity = Some(value);
    }

    /// Gets the value of SoftAntiAffinity
    pub fn get_soft_anti_affinity(&self) -> Option<&u32> {
        self.soft_anti_affinity.as_ref()
    }

/// 

    /// * `name` -  (String)
    /// * `rule_type` -  (u32)

    /// * `created_affinity_rule` -  (MSCluster_AffinityRule)
    /// * `return_value` -  (u32)
    pub fn create_affinity_rule(&self, name: &String, rule_type: u32, created_affinity_rule: &mut MSCluster_AffinityRule) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "RuleType".to_string(), value: rule_type.into() });

        let result = self.invoke_method("CreateAffinityRule", &args)?;
        let created_affinity_rule = result.get_value("CreatedAffinityRule")?;
        Ok(result.return_value)

    }


/// 

    /// * `enabled` -  (u32)
    /// * `rule_type` -  (u32)
    /// * `soft_anti_affinity` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_affinity_rule(&self, rule_type: u32, enabled: u32, soft_anti_affinity: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RuleType".to_string(), value: rule_type.into() });
        args.push(MethodParameter { name: "Enabled".to_string(), value: enabled.into() });
        args.push(MethodParameter { name: "SoftAntiAffinity".to_string(), value: soft_anti_affinity.into() });
        self.invoke_method("SetAffinityRule", &args)

    }


/// 

    /// * `groups` -  (String[])

    /// * `return_value` -  (u32)
    pub fn add_group_to_affinity_rule(&self, groups: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Groups".to_string(), value: groups.into() });
        self.invoke_method("AddGroupToAffinityRule", &args)

    }


/// 

    /// * `groups` -  (String[])

    /// * `return_value` -  (u32)
    pub fn remove_group_from_affinity_rule(&self, groups: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Groups".to_string(), value: groups.into() });
        self.invoke_method("RemoveGroupFromAffinityRule", &args)

    }


/// 

    /// * `cluster_shared_volumes` -  (String[])

    /// * `return_value` -  (u32)
    pub fn add_cluster_shared_volume_to_affinity_rule(&self, cluster_shared_volumes: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ClusterSharedVolumes".to_string(), value: cluster_shared_volumes.into() });
        self.invoke_method("AddClusterSharedVolumeToAffinityRule", &args)

    }


/// 

    /// * `cluster_shared_volumes` -  (String[])

    /// * `return_value` -  (u32)
    pub fn remove_cluster_shared_volume_from_affinity_rule(&self, cluster_shared_volumes: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ClusterSharedVolumes".to_string(), value: cluster_shared_volumes.into() });
        self.invoke_method("RemoveClusterSharedVolumeFromAffinityRule", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn remove_affinity_rule(&self) -> Result<(), WmiError> {
        self.invoke_method("RemoveAffinityRule", &[])

    }

}

