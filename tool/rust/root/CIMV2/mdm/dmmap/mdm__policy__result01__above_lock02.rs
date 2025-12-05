// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_AboveLock02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_AboveLock02 {

/// 
    #[serde(rename = "AllowCortanaAboveLock")]
    pub allow_cortana_above_lock: Option<i32>,

/// 
    #[serde(rename = "AllowToasts")]
    pub allow_toasts: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_Result01_AboveLock02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_cortana_above_lock: None,
            allow_toasts: None,
            instance_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of AllowCortanaAboveLock
    pub fn set_allow_cortana_above_lock(&mut self, value: i32) {
        self.allow_cortana_above_lock = Some(value);
    }

    /// Gets the value of AllowCortanaAboveLock
    pub fn get_allow_cortana_above_lock(&self) -> Option<&i32> {
        self.allow_cortana_above_lock.as_ref()
    }

    /// Sets the value of AllowToasts
    pub fn set_allow_toasts(&mut self, value: i32) {
        self.allow_toasts = Some(value);
    }

    /// Gets the value of AllowToasts
    pub fn get_allow_toasts(&self) -> Option<&i32> {
        self.allow_toasts.as_ref()
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

