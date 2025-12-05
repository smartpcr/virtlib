// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_eUICCs_Profiles02_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_eUICCs_Profiles02_01 {

/// 
    #[serde(rename = "ErrorDetail")]
    pub error_detail: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "IsEnabled")]
    pub is_enabled: Option<bool>,

/// 
    #[serde(rename = "MatchingID")]
    pub matching_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PPR1Set")]
    pub ppr1_set: Option<bool>,

/// 
    #[serde(rename = "PPR2Set")]
    pub ppr2_set: Option<bool>,

/// 
    #[serde(rename = "ServerName")]
    pub server_name: Option<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<i32>,
}

impl MDM_eUICCs_Profiles02_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            error_detail: None,
            instance_id: None,
            is_enabled: None,
            matching_id: None,
            parent_id: None,
            ppr1_set: None,
            ppr2_set: None,
            server_name: None,
            state: None,
        }
    }


    /// Sets the value of ErrorDetail
    pub fn set_error_detail(&mut self, value: i32) {
        self.error_detail = Some(value);
    }

    /// Gets the value of ErrorDetail
    pub fn get_error_detail(&self) -> Option<&i32> {
        self.error_detail.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of IsEnabled
    pub fn set_is_enabled(&mut self, value: bool) {
        self.is_enabled = Some(value);
    }

    /// Gets the value of IsEnabled
    pub fn get_is_enabled(&self) -> Option<&bool> {
        self.is_enabled.as_ref()
    }

    /// Sets the value of MatchingID
    pub fn set_matching_id(&mut self, value: String) {
        self.matching_id = Some(value);
    }

    /// Gets the value of MatchingID
    pub fn get_matching_id(&self) -> Option<&String> {
        self.matching_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PPR1Set
    pub fn set_ppr1_set(&mut self, value: bool) {
        self.ppr1_set = Some(value);
    }

    /// Gets the value of PPR1Set
    pub fn get_ppr1_set(&self) -> Option<&bool> {
        self.ppr1_set.as_ref()
    }

    /// Sets the value of PPR2Set
    pub fn set_ppr2_set(&mut self, value: bool) {
        self.ppr2_set = Some(value);
    }

    /// Gets the value of PPR2Set
    pub fn get_ppr2_set(&self) -> Option<&bool> {
        self.ppr2_set.as_ref()
    }

    /// Sets the value of ServerName
    pub fn set_server_name(&mut self, value: String) {
        self.server_name = Some(value);
    }

    /// Gets the value of ServerName
    pub fn get_server_name(&self) -> Option<&String> {
        self.server_name.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: i32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&i32> {
        self.state.as_ref()
    }
}

