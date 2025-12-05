// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetFirewallRule struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetFirewallRule {
    #[serde(flatten)]
    pub base: CIM_PolicyRule,

/// 
    #[serde(rename = "Action")]
    pub action: Option<u16>,

/// 
    #[serde(rename = "Direction")]
    pub direction: Option<u16>,

/// 
    #[serde(rename = "DisplayGroup")]
    pub display_group: Option<String>,

/// 
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "EdgeTraversalPolicy")]
    pub edge_traversal_policy: Option<u16>,

/// 
    #[serde(rename = "EnforcementStatus")]
    pub enforcement_status: Vec<u16>,

/// 
    #[serde(rename = "LocalOnlyMapping")]
    pub local_only_mapping: Option<bool>,

/// 
    #[serde(rename = "LooseSourceMapping")]
    pub loose_source_mapping: Option<bool>,

/// 
    #[serde(rename = "Owner")]
    pub owner: Option<String>,

/// 
    #[serde(rename = "PackageFamilyName")]
    pub package_family_name: Option<String>,

/// 
    #[serde(rename = "Platforms")]
    pub platforms: Vec<String>,

/// 
    #[serde(rename = "PolicyAppId")]
    pub policy_app_id: Option<String>,

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
    #[serde(rename = "RemoteDynamicKeywordAddresses")]
    pub remote_dynamic_keyword_addresses: Vec<String>,

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

impl MSFT_NetFirewallRule {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PolicyRule::new(),
            action: None,
            direction: None,
            display_group: None,
            display_name: None,
            edge_traversal_policy: None,
            enforcement_status: Vec::new(),
            local_only_mapping: None,
            loose_source_mapping: None,
            owner: None,
            package_family_name: None,
            platforms: Vec::new(),
            policy_app_id: None,
            policy_store_source: None,
            policy_store_source_type: None,
            primary_status: None,
            profiles: None,
            remote_dynamic_keyword_addresses: Vec::new(),
            rule_group: None,
            status: None,
            status_code: None,
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

    /// Sets the value of Direction
    pub fn set_direction(&mut self, value: u16) {
        self.direction = Some(value);
    }

    /// Gets the value of Direction
    pub fn get_direction(&self) -> Option<&u16> {
        self.direction.as_ref()
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

    /// Sets the value of EdgeTraversalPolicy
    pub fn set_edge_traversal_policy(&mut self, value: u16) {
        self.edge_traversal_policy = Some(value);
    }

    /// Gets the value of EdgeTraversalPolicy
    pub fn get_edge_traversal_policy(&self) -> Option<&u16> {
        self.edge_traversal_policy.as_ref()
    }

    /// Sets the value of EnforcementStatus
    pub fn set_enforcement_status(&mut self, value: Vec<u16>) {
        self.enforcement_status = value;
    }

    /// Gets the value of EnforcementStatus
    pub fn get_enforcement_status(&self) -> &Vec<u16> {
        &self.enforcement_status
    }

    /// Sets the value of LocalOnlyMapping
    pub fn set_local_only_mapping(&mut self, value: bool) {
        self.local_only_mapping = Some(value);
    }

    /// Gets the value of LocalOnlyMapping
    pub fn get_local_only_mapping(&self) -> Option<&bool> {
        self.local_only_mapping.as_ref()
    }

    /// Sets the value of LooseSourceMapping
    pub fn set_loose_source_mapping(&mut self, value: bool) {
        self.loose_source_mapping = Some(value);
    }

    /// Gets the value of LooseSourceMapping
    pub fn get_loose_source_mapping(&self) -> Option<&bool> {
        self.loose_source_mapping.as_ref()
    }

    /// Sets the value of Owner
    pub fn set_owner(&mut self, value: String) {
        self.owner = Some(value);
    }

    /// Gets the value of Owner
    pub fn get_owner(&self) -> Option<&String> {
        self.owner.as_ref()
    }

    /// Sets the value of PackageFamilyName
    pub fn set_package_family_name(&mut self, value: String) {
        self.package_family_name = Some(value);
    }

    /// Gets the value of PackageFamilyName
    pub fn get_package_family_name(&self) -> Option<&String> {
        self.package_family_name.as_ref()
    }

    /// Sets the value of Platforms
    pub fn set_platforms(&mut self, value: Vec<String>) {
        self.platforms = value;
    }

    /// Gets the value of Platforms
    pub fn get_platforms(&self) -> &Vec<String> {
        &self.platforms
    }

    /// Sets the value of PolicyAppId
    pub fn set_policy_app_id(&mut self, value: String) {
        self.policy_app_id = Some(value);
    }

    /// Gets the value of PolicyAppId
    pub fn get_policy_app_id(&self) -> Option<&String> {
        self.policy_app_id.as_ref()
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

    /// Sets the value of RemoteDynamicKeywordAddresses
    pub fn set_remote_dynamic_keyword_addresses(&mut self, value: Vec<String>) {
        self.remote_dynamic_keyword_addresses = value;
    }

    /// Gets the value of RemoteDynamicKeywordAddresses
    pub fn get_remote_dynamic_keyword_addresses(&self) -> &Vec<String> {
        &self.remote_dynamic_keyword_addresses
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

/// 

    /// * `return_value` -  (u32)
    pub fn enable(&self) -> Result<(), WmiError> {
        self.invoke_method("Enable", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn disable(&self) -> Result<(), WmiError> {
        self.invoke_method("Disable", &[])

    }


/// 

    /// * `new_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn rename(&self, new_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });
        self.invoke_method("Rename", &args)

    }


/// 

    /// * `new_gposession` -  (String)
    /// * `new_name` -  (String)
    /// * `new_policy_store` -  (String)

    /// * `return_value` -  (u32)
    pub fn clone_object(&self, new_name: &String, new_policy_store: &String, new_gposession: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });
        args.push(MethodParameter { name: "NewPolicyStore".to_string(), value: new_policy_store.into() });
        args.push(MethodParameter { name: "NewGPOSession".to_string(), value: new_gposession.into() });
        self.invoke_method("CloneObject", &args)

    }


/// 

    /// * `dependents` -  (CIM_ManagedSystemElement[])
    /// * `return_value` -  (u32)
    pub fn enumerate_full(&self, dependents: &mut Vec<CIM_ManagedSystemElement>) -> Result<(), WmiError> {

        let result = self.invoke_method("EnumerateFull", &[])?;
        let dependents = result.get_value("Dependents")?;
        Ok(result.return_value)

    }

}

