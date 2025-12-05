// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_DataProtection02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_DataProtection02 {

/// 
    #[serde(rename = "AllowDirectMemoryAccess")]
    pub allow_direct_memory_access: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LegacySelectiveWipeID")]
    pub legacy_selective_wipe_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_Result01_DataProtection02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_direct_memory_access: None,
            instance_id: None,
            legacy_selective_wipe_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of AllowDirectMemoryAccess
    pub fn set_allow_direct_memory_access(&mut self, value: i32) {
        self.allow_direct_memory_access = Some(value);
    }

    /// Gets the value of AllowDirectMemoryAccess
    pub fn get_allow_direct_memory_access(&self) -> Option<&i32> {
        self.allow_direct_memory_access.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of LegacySelectiveWipeID
    pub fn set_legacy_selective_wipe_id(&mut self, value: String) {
        self.legacy_selective_wipe_id = Some(value);
    }

    /// Gets the value of LegacySelectiveWipeID
    pub fn get_legacy_selective_wipe_id(&self) -> Option<&String> {
        self.legacy_selective_wipe_id.as_ref()
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

