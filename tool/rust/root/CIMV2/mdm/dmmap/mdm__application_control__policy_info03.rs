// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_ApplicationControl_PolicyInfo03 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_ApplicationControl_PolicyInfo03 {

/// 
    #[serde(rename = "BasePolicyId")]
    pub base_policy_id: Option<String>,

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "IsAuthorized")]
    pub is_authorized: Option<bool>,

/// 
    #[serde(rename = "IsBasePolicy")]
    pub is_base_policy: Option<bool>,

/// 
    #[serde(rename = "IsDeployed")]
    pub is_deployed: Option<bool>,

/// 
    #[serde(rename = "IsEffective")]
    pub is_effective: Option<bool>,

/// 
    #[serde(rename = "IsSystemPolicy")]
    pub is_system_policy: Option<bool>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PolicyOptions")]
    pub policy_options: Option<String>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<i32>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl MDM_ApplicationControl_PolicyInfo03 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base_policy_id: None,
            friendly_name: None,
            instance_id: None,
            is_authorized: None,
            is_base_policy: None,
            is_deployed: None,
            is_effective: None,
            is_system_policy: None,
            parent_id: None,
            policy_options: None,
            status: None,
            version: None,
        }
    }


    /// Sets the value of BasePolicyId
    pub fn set_base_policy_id(&mut self, value: String) {
        self.base_policy_id = Some(value);
    }

    /// Gets the value of BasePolicyId
    pub fn get_base_policy_id(&self) -> Option<&String> {
        self.base_policy_id.as_ref()
    }

    /// Sets the value of FriendlyName
    pub fn set_friendly_name(&mut self, value: String) {
        self.friendly_name = Some(value);
    }

    /// Gets the value of FriendlyName
    pub fn get_friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of IsAuthorized
    pub fn set_is_authorized(&mut self, value: bool) {
        self.is_authorized = Some(value);
    }

    /// Gets the value of IsAuthorized
    pub fn get_is_authorized(&self) -> Option<&bool> {
        self.is_authorized.as_ref()
    }

    /// Sets the value of IsBasePolicy
    pub fn set_is_base_policy(&mut self, value: bool) {
        self.is_base_policy = Some(value);
    }

    /// Gets the value of IsBasePolicy
    pub fn get_is_base_policy(&self) -> Option<&bool> {
        self.is_base_policy.as_ref()
    }

    /// Sets the value of IsDeployed
    pub fn set_is_deployed(&mut self, value: bool) {
        self.is_deployed = Some(value);
    }

    /// Gets the value of IsDeployed
    pub fn get_is_deployed(&self) -> Option<&bool> {
        self.is_deployed.as_ref()
    }

    /// Sets the value of IsEffective
    pub fn set_is_effective(&mut self, value: bool) {
        self.is_effective = Some(value);
    }

    /// Gets the value of IsEffective
    pub fn get_is_effective(&self) -> Option<&bool> {
        self.is_effective.as_ref()
    }

    /// Sets the value of IsSystemPolicy
    pub fn set_is_system_policy(&mut self, value: bool) {
        self.is_system_policy = Some(value);
    }

    /// Gets the value of IsSystemPolicy
    pub fn get_is_system_policy(&self) -> Option<&bool> {
        self.is_system_policy.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PolicyOptions
    pub fn set_policy_options(&mut self, value: String) {
        self.policy_options = Some(value);
    }

    /// Gets the value of PolicyOptions
    pub fn get_policy_options(&self) -> Option<&String> {
        self.policy_options.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: i32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&i32> {
        self.status.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: String) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }
}

