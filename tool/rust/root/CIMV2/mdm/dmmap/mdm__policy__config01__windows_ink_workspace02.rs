// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_WindowsInkWorkspace02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_WindowsInkWorkspace02 {

/// 
    #[serde(rename = "AllowSuggestedAppsInWindowsInkWorkspace")]
    pub allow_suggested_apps_in_windows_ink_workspace: Option<i32>,

/// 
    #[serde(rename = "AllowWindowsInkWorkspace")]
    pub allow_windows_ink_workspace: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_Config01_WindowsInkWorkspace02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_suggested_apps_in_windows_ink_workspace: None,
            allow_windows_ink_workspace: None,
            instance_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of AllowSuggestedAppsInWindowsInkWorkspace
    pub fn set_allow_suggested_apps_in_windows_ink_workspace(&mut self, value: i32) {
        self.allow_suggested_apps_in_windows_ink_workspace = Some(value);
    }

    /// Gets the value of AllowSuggestedAppsInWindowsInkWorkspace
    pub fn get_allow_suggested_apps_in_windows_ink_workspace(&self) -> Option<&i32> {
        self.allow_suggested_apps_in_windows_ink_workspace.as_ref()
    }

    /// Sets the value of AllowWindowsInkWorkspace
    pub fn set_allow_windows_ink_workspace(&mut self, value: i32) {
        self.allow_windows_ink_workspace = Some(value);
    }

    /// Gets the value of AllowWindowsInkWorkspace
    pub fn get_allow_windows_ink_workspace(&self) -> Option<&i32> {
        self.allow_windows_ink_workspace.as_ref()
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

