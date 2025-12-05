// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ISCSI_RedirectSessionInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ISCSI_RedirectSessionInfo {

/// Number of elements in RedirectPortalList array
    #[serde(rename = "ConnectionCount")]
    pub connection_count: Option<u32>,

/// Redirect portal info - one element for each connection in the session
    #[serde(rename = "RedirectPortalList")]
    pub redirect_portal_list: Vec<ISCSI_RedirectPortalInfo>,

/// Target portal group tag for this Session 
    #[serde(rename = "TargetPortalGroupTag")]
    pub target_portal_group_tag: Option<u32>,

/// A uniquely generated session ID, it is the same id returned by the LoginToTarget method.  Do not confuse this with ISID or SSID.
    #[serde(rename = "UniqueSessionId")]
    pub unique_session_id: Option<u64>,
}

impl ISCSI_RedirectSessionInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            connection_count: None,
            redirect_portal_list: Vec::new(),
            target_portal_group_tag: None,
            unique_session_id: None,
        }
    }


    /// Sets the value of ConnectionCount
    pub fn set_connection_count(&mut self, value: u32) {
        self.connection_count = Some(value);
    }

    /// Gets the value of ConnectionCount
    pub fn get_connection_count(&self) -> Option<&u32> {
        self.connection_count.as_ref()
    }

    /// Sets the value of RedirectPortalList
    pub fn set_redirect_portal_list(&mut self, value: Vec<ISCSI_RedirectPortalInfo>) {
        self.redirect_portal_list = value;
    }

    /// Gets the value of RedirectPortalList
    pub fn get_redirect_portal_list(&self) -> &Vec<ISCSI_RedirectPortalInfo> {
        &self.redirect_portal_list
    }

    /// Sets the value of TargetPortalGroupTag
    pub fn set_target_portal_group_tag(&mut self, value: u32) {
        self.target_portal_group_tag = Some(value);
    }

    /// Gets the value of TargetPortalGroupTag
    pub fn get_target_portal_group_tag(&self) -> Option<&u32> {
        self.target_portal_group_tag.as_ref()
    }

    /// Sets the value of UniqueSessionId
    pub fn set_unique_session_id(&mut self, value: u64) {
        self.unique_session_id = Some(value);
    }

    /// Gets the value of UniqueSessionId
    pub fn get_unique_session_id(&self) -> Option<&u64> {
        self.unique_session_id.as_ref()
    }
}

