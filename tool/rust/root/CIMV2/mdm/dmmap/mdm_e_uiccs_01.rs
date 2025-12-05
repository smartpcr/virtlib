// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_eUICCs_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_eUICCs_01 {

/// 
    #[serde(rename = "Identifier")]
    pub identifier: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "IsActive")]
    pub is_active: Option<bool>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PPR1Allowed")]
    pub ppr1_allowed: Option<bool>,

/// 
    #[serde(rename = "PPR1AlreadySet")]
    pub ppr1_already_set: Option<bool>,
}

impl MDM_eUICCs_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            identifier: None,
            instance_id: None,
            is_active: None,
            parent_id: None,
            ppr1_allowed: None,
            ppr1_already_set: None,
        }
    }


    /// Sets the value of Identifier
    pub fn set_identifier(&mut self, value: String) {
        self.identifier = Some(value);
    }

    /// Gets the value of Identifier
    pub fn get_identifier(&self) -> Option<&String> {
        self.identifier.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of IsActive
    pub fn set_is_active(&mut self, value: bool) {
        self.is_active = Some(value);
    }

    /// Gets the value of IsActive
    pub fn get_is_active(&self) -> Option<&bool> {
        self.is_active.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PPR1Allowed
    pub fn set_ppr1_allowed(&mut self, value: bool) {
        self.ppr1_allowed = Some(value);
    }

    /// Gets the value of PPR1Allowed
    pub fn get_ppr1_allowed(&self) -> Option<&bool> {
        self.ppr1_allowed.as_ref()
    }

    /// Sets the value of PPR1AlreadySet
    pub fn set_ppr1_already_set(&mut self, value: bool) {
        self.ppr1_already_set = Some(value);
    }

    /// Gets the value of PPR1AlreadySet
    pub fn get_ppr1_already_set(&self) -> Option<&bool> {
        self.ppr1_already_set.as_ref()
    }
}

