// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSI_RedirectPortalInfoClass struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSI_RedirectPortalInfoClass {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// Variable length array of ISCSI_RedirectSessionInfo. SessionCount specifies the number of elements in the array. NOTE: this is a variable length array.
    #[serde(rename = "RedirectSessionList")]
    pub redirect_session_list: Vec<ISCSI_RedirectSessionInfo>,

/// Number of elements in RedirectSessionInfo array
    #[serde(rename = "SessionCount")]
    pub session_count: Option<u32>,

/// Id that is globally unique for all instances of iSCSI initiators.
    #[serde(rename = "UniqueAdapterId")]
    pub unique_adapter_id: Option<u64>,
}

impl MSiSCSI_RedirectPortalInfoClass {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            instance_name: None,
            redirect_session_list: Vec::new(),
            session_count: None,
            unique_adapter_id: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of RedirectSessionList
    pub fn set_redirect_session_list(&mut self, value: Vec<ISCSI_RedirectSessionInfo>) {
        self.redirect_session_list = value;
    }

    /// Gets the value of RedirectSessionList
    pub fn get_redirect_session_list(&self) -> &Vec<ISCSI_RedirectSessionInfo> {
        &self.redirect_session_list
    }

    /// Sets the value of SessionCount
    pub fn set_session_count(&mut self, value: u32) {
        self.session_count = Some(value);
    }

    /// Gets the value of SessionCount
    pub fn get_session_count(&self) -> Option<&u32> {
        self.session_count.as_ref()
    }

    /// Sets the value of UniqueAdapterId
    pub fn set_unique_adapter_id(&mut self, value: u64) {
        self.unique_adapter_id = Some(value);
    }

    /// Gets the value of UniqueAdapterId
    pub fn get_unique_adapter_id(&self) -> Option<&u64> {
        self.unique_adapter_id.as_ref()
    }
}

