// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_AppLocker_DLL03 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_AppLocker_DLL03 {

/// 
    #[serde(rename = "EnforcementMode")]
    pub enforcement_mode: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "NonInteractiveProcessEnforcement")]
    pub non_interactive_process_enforcement: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "Policy")]
    pub policy: Option<String>,
}

impl MDM_AppLocker_DLL03 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            enforcement_mode: None,
            instance_id: None,
            non_interactive_process_enforcement: None,
            parent_id: None,
            policy: None,
        }
    }


    /// Sets the value of EnforcementMode
    pub fn set_enforcement_mode(&mut self, value: String) {
        self.enforcement_mode = Some(value);
    }

    /// Gets the value of EnforcementMode
    pub fn get_enforcement_mode(&self) -> Option<&String> {
        self.enforcement_mode.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of NonInteractiveProcessEnforcement
    pub fn set_non_interactive_process_enforcement(&mut self, value: String) {
        self.non_interactive_process_enforcement = Some(value);
    }

    /// Gets the value of NonInteractiveProcessEnforcement
    pub fn get_non_interactive_process_enforcement(&self) -> Option<&String> {
        self.non_interactive_process_enforcement.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of Policy
    pub fn set_policy(&mut self, value: String) {
        self.policy = Some(value);
    }

    /// Gets the value of Policy
    pub fn get_policy(&self) -> Option<&String> {
        self.policy.as_ref()
    }
}

