// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_PassportForWork_ExcludeSecurityDevices03 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_PassportForWork_ExcludeSecurityDevices03 {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "TPM12")]
    pub tpm12: Option<bool>,
}

impl MDM_PassportForWork_ExcludeSecurityDevices03 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            parent_id: None,
            tpm12: None,
        }
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

    /// Sets the value of TPM12
    pub fn set_tpm12(&mut self, value: bool) {
        self.tpm12 = Some(value);
    }

    /// Gets the value of TPM12
    pub fn get_tpm12(&self) -> Option<&bool> {
        self.tpm12.as_ref()
    }
}

