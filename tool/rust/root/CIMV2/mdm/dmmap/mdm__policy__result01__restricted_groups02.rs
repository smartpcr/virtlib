// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_RestrictedGroups02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_RestrictedGroups02 {

/// 
    #[serde(rename = "ConfigureGroupMembership")]
    pub configure_group_membership: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_Result01_RestrictedGroups02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            configure_group_membership: None,
            instance_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of ConfigureGroupMembership
    pub fn set_configure_group_membership(&mut self, value: String) {
        self.configure_group_membership = Some(value);
    }

    /// Gets the value of ConfigureGroupMembership
    pub fn get_configure_group_membership(&self) -> Option<&String> {
        self.configure_group_membership.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }
}

