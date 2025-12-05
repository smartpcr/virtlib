// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_DevicePreparation_MDMProvider01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_DevicePreparation_MDMProvider01 {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "MdmAgentInstalled")]
    pub mdm_agent_installed: Option<bool>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "Progress")]
    pub progress: Option<String>,

/// 
    #[serde(rename = "RebootRequired")]
    pub reboot_required: Option<bool>,
}

impl MDM_DevicePreparation_MDMProvider01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            mdm_agent_installed: None,
            parent_id: None,
            progress: None,
            reboot_required: None,
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

    /// Sets the value of MdmAgentInstalled
    pub fn set_mdm_agent_installed(&mut self, value: bool) {
        self.mdm_agent_installed = Some(value);
    }

    /// Gets the value of MdmAgentInstalled
    pub fn get_mdm_agent_installed(&self) -> Option<&bool> {
        self.mdm_agent_installed.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of Progress
    pub fn set_progress(&mut self, value: String) {
        self.progress = Some(value);
    }

    /// Gets the value of Progress
    pub fn get_progress(&self) -> Option<&String> {
        self.progress.as_ref()
    }

    /// Sets the value of RebootRequired
    pub fn set_reboot_required(&mut self, value: bool) {
        self.reboot_required = Some(value);
    }

    /// Gets the value of RebootRequired
    pub fn get_reboot_required(&self) -> Option<&bool> {
        self.reboot_required.as_ref()
    }
}

