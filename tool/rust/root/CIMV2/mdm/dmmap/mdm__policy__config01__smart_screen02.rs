// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_SmartScreen02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_SmartScreen02 {

/// 
    #[serde(rename = "EnableAppInstallControl")]
    pub enable_app_install_control: Option<i32>,

/// 
    #[serde(rename = "EnableSmartScreenInShell")]
    pub enable_smart_screen_in_shell: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PreventOverrideForFilesInShell")]
    pub prevent_override_for_files_in_shell: Option<i32>,
}

impl MDM_Policy_Config01_SmartScreen02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            enable_app_install_control: None,
            enable_smart_screen_in_shell: None,
            instance_id: None,
            parent_id: None,
            prevent_override_for_files_in_shell: None,
        }
    }


    /// Sets the value of EnableAppInstallControl
    pub fn set_enable_app_install_control(&mut self, value: i32) {
        self.enable_app_install_control = Some(value);
    }

    /// Gets the value of EnableAppInstallControl
    pub fn get_enable_app_install_control(&self) -> Option<&i32> {
        self.enable_app_install_control.as_ref()
    }

    /// Sets the value of EnableSmartScreenInShell
    pub fn set_enable_smart_screen_in_shell(&mut self, value: i32) {
        self.enable_smart_screen_in_shell = Some(value);
    }

    /// Gets the value of EnableSmartScreenInShell
    pub fn get_enable_smart_screen_in_shell(&self) -> Option<&i32> {
        self.enable_smart_screen_in_shell.as_ref()
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

    /// Sets the value of PreventOverrideForFilesInShell
    pub fn set_prevent_override_for_files_in_shell(&mut self, value: i32) {
        self.prevent_override_for_files_in_shell = Some(value);
    }

    /// Gets the value of PreventOverrideForFilesInShell
    pub fn get_prevent_override_for_files_in_shell(&self) -> Option<&i32> {
        self.prevent_override_for_files_in_shell.as_ref()
    }
}

