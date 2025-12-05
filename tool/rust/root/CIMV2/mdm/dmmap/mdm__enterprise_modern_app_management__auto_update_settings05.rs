// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_EnterpriseModernAppManagement_AutoUpdateSettings05 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_EnterpriseModernAppManagement_AutoUpdateSettings05 {

/// 
    #[serde(rename = "AutomaticBackgroundTask")]
    pub automatic_background_task: Option<bool>,

/// 
    #[serde(rename = "Disable")]
    pub disable: Option<bool>,

/// 
    #[serde(rename = "ForceUpdateFromAnyVersion")]
    pub force_update_from_any_version: Option<bool>,

/// 
    #[serde(rename = "HoursBetweenUpdateChecks")]
    pub hours_between_update_checks: Option<bool>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "OnLaunchUpdateCheck")]
    pub on_launch_update_check: Option<bool>,

/// 
    #[serde(rename = "PackageSource")]
    pub package_source: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "ShowPrompt")]
    pub show_prompt: Option<bool>,

/// 
    #[serde(rename = "UpdateBlocksActivation")]
    pub update_blocks_activation: Option<bool>,
}

impl MDM_EnterpriseModernAppManagement_AutoUpdateSettings05 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            automatic_background_task: None,
            disable: None,
            force_update_from_any_version: None,
            hours_between_update_checks: None,
            instance_id: None,
            on_launch_update_check: None,
            package_source: None,
            parent_id: None,
            show_prompt: None,
            update_blocks_activation: None,
        }
    }


    /// Sets the value of AutomaticBackgroundTask
    pub fn set_automatic_background_task(&mut self, value: bool) {
        self.automatic_background_task = Some(value);
    }

    /// Gets the value of AutomaticBackgroundTask
    pub fn get_automatic_background_task(&self) -> Option<&bool> {
        self.automatic_background_task.as_ref()
    }

    /// Sets the value of Disable
    pub fn set_disable(&mut self, value: bool) {
        self.disable = Some(value);
    }

    /// Gets the value of Disable
    pub fn get_disable(&self) -> Option<&bool> {
        self.disable.as_ref()
    }

    /// Sets the value of ForceUpdateFromAnyVersion
    pub fn set_force_update_from_any_version(&mut self, value: bool) {
        self.force_update_from_any_version = Some(value);
    }

    /// Gets the value of ForceUpdateFromAnyVersion
    pub fn get_force_update_from_any_version(&self) -> Option<&bool> {
        self.force_update_from_any_version.as_ref()
    }

    /// Sets the value of HoursBetweenUpdateChecks
    pub fn set_hours_between_update_checks(&mut self, value: bool) {
        self.hours_between_update_checks = Some(value);
    }

    /// Gets the value of HoursBetweenUpdateChecks
    pub fn get_hours_between_update_checks(&self) -> Option<&bool> {
        self.hours_between_update_checks.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of OnLaunchUpdateCheck
    pub fn set_on_launch_update_check(&mut self, value: bool) {
        self.on_launch_update_check = Some(value);
    }

    /// Gets the value of OnLaunchUpdateCheck
    pub fn get_on_launch_update_check(&self) -> Option<&bool> {
        self.on_launch_update_check.as_ref()
    }

    /// Sets the value of PackageSource
    pub fn set_package_source(&mut self, value: String) {
        self.package_source = Some(value);
    }

    /// Gets the value of PackageSource
    pub fn get_package_source(&self) -> Option<&String> {
        self.package_source.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of ShowPrompt
    pub fn set_show_prompt(&mut self, value: bool) {
        self.show_prompt = Some(value);
    }

    /// Gets the value of ShowPrompt
    pub fn get_show_prompt(&self) -> Option<&bool> {
        self.show_prompt.as_ref()
    }

    /// Sets the value of UpdateBlocksActivation
    pub fn set_update_blocks_activation(&mut self, value: bool) {
        self.update_blocks_activation = Some(value);
    }

    /// Gets the value of UpdateBlocksActivation
    pub fn get_update_blocks_activation(&self) -> Option<&bool> {
        self.update_blocks_activation.as_ref()
    }
}

