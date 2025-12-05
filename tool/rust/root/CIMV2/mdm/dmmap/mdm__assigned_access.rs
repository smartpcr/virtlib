// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_AssignedAccess struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_AssignedAccess {

/// 
    #[serde(rename = "Configuration")]
    pub configuration: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "KioskModeApp")]
    pub kiosk_mode_app: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "ShellLauncher")]
    pub shell_launcher: Option<String>,
}

impl MDM_AssignedAccess {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            configuration: None,
            instance_id: None,
            kiosk_mode_app: None,
            parent_id: None,
            shell_launcher: None,
        }
    }


    /// Sets the value of Configuration
    pub fn set_configuration(&mut self, value: String) {
        self.configuration = Some(value);
    }

    /// Gets the value of Configuration
    pub fn get_configuration(&self) -> Option<&String> {
        self.configuration.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of KioskModeApp
    pub fn set_kiosk_mode_app(&mut self, value: String) {
        self.kiosk_mode_app = Some(value);
    }

    /// Gets the value of KioskModeApp
    pub fn get_kiosk_mode_app(&self) -> Option<&String> {
        self.kiosk_mode_app.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of ShellLauncher
    pub fn set_shell_launcher(&mut self, value: String) {
        self.shell_launcher = Some(value);
    }

    /// Gets the value of ShellLauncher
    pub fn get_shell_launcher(&self) -> Option<&String> {
        self.shell_launcher.as_ref()
    }
}

