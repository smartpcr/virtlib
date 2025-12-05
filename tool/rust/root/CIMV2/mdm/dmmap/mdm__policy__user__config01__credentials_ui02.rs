// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_User_Config01_CredentialsUI02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_User_Config01_CredentialsUI02 {

/// 
    #[serde(rename = "DisablePasswordReveal")]
    pub disable_password_reveal: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_User_Config01_CredentialsUI02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            disable_password_reveal: None,
            instance_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of DisablePasswordReveal
    pub fn set_disable_password_reveal(&mut self, value: String) {
        self.disable_password_reveal = Some(value);
    }

    /// Gets the value of DisablePasswordReveal
    pub fn get_disable_password_reveal(&self) -> Option<&String> {
        self.disable_password_reveal.as_ref()
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

