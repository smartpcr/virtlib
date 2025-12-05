// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_EnterpriseAPN_Settings01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_EnterpriseAPN_Settings01 {

/// 
    #[serde(rename = "AllowUserControl")]
    pub allow_user_control: Option<bool>,

/// 
    #[serde(rename = "HideView")]
    pub hide_view: Option<bool>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_EnterpriseAPN_Settings01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_user_control: None,
            hide_view: None,
            instance_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of AllowUserControl
    pub fn set_allow_user_control(&mut self, value: bool) {
        self.allow_user_control = Some(value);
    }

    /// Gets the value of AllowUserControl
    pub fn get_allow_user_control(&self) -> Option<&bool> {
        self.allow_user_control.as_ref()
    }

    /// Sets the value of HideView
    pub fn set_hide_view(&mut self, value: bool) {
        self.hide_view = Some(value);
    }

    /// Gets the value of HideView
    pub fn get_hide_view(&self) -> Option<&bool> {
        self.hide_view.as_ref()
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

