// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_Cellular02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_Cellular02 {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessCellularData")]
    pub let_apps_access_cellular_data: Option<i32>,

/// 
    #[serde(rename = "LetAppsAccessCellularData_ForceAllowTheseApps")]
    pub let_apps_access_cellular_data__force_allow_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessCellularData_ForceDenyTheseApps")]
    pub let_apps_access_cellular_data__force_deny_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessCellularData_UserInControlOfTheseApps")]
    pub let_apps_access_cellular_data__user_in_control_of_these_apps: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "ShowAppCellularAccessUI")]
    pub show_app_cellular_access_ui: Option<String>,
}

impl MDM_Policy_Config01_Cellular02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            let_apps_access_cellular_data: None,
            let_apps_access_cellular_data__force_allow_these_apps: None,
            let_apps_access_cellular_data__force_deny_these_apps: None,
            let_apps_access_cellular_data__user_in_control_of_these_apps: None,
            parent_id: None,
            show_app_cellular_access_ui: None,
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

    /// Sets the value of LetAppsAccessCellularData
    pub fn set_let_apps_access_cellular_data(&mut self, value: i32) {
        self.let_apps_access_cellular_data = Some(value);
    }

    /// Gets the value of LetAppsAccessCellularData
    pub fn get_let_apps_access_cellular_data(&self) -> Option<&i32> {
        self.let_apps_access_cellular_data.as_ref()
    }

    /// Sets the value of LetAppsAccessCellularData_ForceAllowTheseApps
    pub fn set_let_apps_access_cellular_data__force_allow_these_apps(&mut self, value: String) {
        self.let_apps_access_cellular_data__force_allow_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessCellularData_ForceAllowTheseApps
    pub fn get_let_apps_access_cellular_data__force_allow_these_apps(&self) -> Option<&String> {
        self.let_apps_access_cellular_data__force_allow_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessCellularData_ForceDenyTheseApps
    pub fn set_let_apps_access_cellular_data__force_deny_these_apps(&mut self, value: String) {
        self.let_apps_access_cellular_data__force_deny_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessCellularData_ForceDenyTheseApps
    pub fn get_let_apps_access_cellular_data__force_deny_these_apps(&self) -> Option<&String> {
        self.let_apps_access_cellular_data__force_deny_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessCellularData_UserInControlOfTheseApps
    pub fn set_let_apps_access_cellular_data__user_in_control_of_these_apps(&mut self, value: String) {
        self.let_apps_access_cellular_data__user_in_control_of_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessCellularData_UserInControlOfTheseApps
    pub fn get_let_apps_access_cellular_data__user_in_control_of_these_apps(&self) -> Option<&String> {
        self.let_apps_access_cellular_data__user_in_control_of_these_apps.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of ShowAppCellularAccessUI
    pub fn set_show_app_cellular_access_ui(&mut self, value: String) {
        self.show_app_cellular_access_ui = Some(value);
    }

    /// Gets the value of ShowAppCellularAccessUI
    pub fn get_show_app_cellular_access_ui(&self) -> Option<&String> {
        self.show_app_cellular_access_ui.as_ref()
    }
}

