// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_UserModel02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_UserModel02 {

/// 
    #[serde(rename = "EnterpriseSupport")]
    pub enterprise_support: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "UserModel")]
    pub user_model: Option<i32>,
}

impl MDM_Policy_Config01_UserModel02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            enterprise_support: None,
            instance_id: None,
            parent_id: None,
            user_model: None,
        }
    }


    /// Sets the value of EnterpriseSupport
    pub fn set_enterprise_support(&mut self, value: String) {
        self.enterprise_support = Some(value);
    }

    /// Gets the value of EnterpriseSupport
    pub fn get_enterprise_support(&self) -> Option<&String> {
        self.enterprise_support.as_ref()
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

    /// Sets the value of UserModel
    pub fn set_user_model(&mut self, value: i32) {
        self.user_model = Some(value);
    }

    /// Gets the value of UserModel
    pub fn get_user_model(&self) -> Option<&i32> {
        self.user_model.as_ref()
    }
}

