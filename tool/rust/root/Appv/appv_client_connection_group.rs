// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Appv
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// AppvClientConnectionGroup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppvClientConnectionGroup {

/// 
    #[serde(rename = "CustomData")]
    pub custom_data: Option<String>,

/// 
    #[serde(rename = "GlobalPending")]
    pub global_pending: Option<bool>,

/// 
    #[serde(rename = "GroupId")]
    pub group_id: Option<String>,

/// 
    #[serde(rename = "InUse")]
    pub in_use: Option<bool>,

/// 
    #[serde(rename = "IsEnabledGlobally")]
    pub is_enabled_globally: Option<bool>,

/// 
    #[serde(rename = "IsEnabledToUser")]
    pub is_enabled_to_user: Option<bool>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Packages")]
    pub packages: Vec<String>,

/// 
    #[serde(rename = "PercentLoaded")]
    pub percent_loaded: Option<u16>,

/// 
    #[serde(rename = "Priority")]
    pub priority: Option<u32>,

/// 
    #[serde(rename = "UserPending")]
    pub user_pending: Option<bool>,

/// 
    #[serde(rename = "VersionId")]
    pub version_id: Option<String>,
}

impl AppvClientConnectionGroup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            custom_data: None,
            global_pending: None,
            group_id: None,
            in_use: None,
            is_enabled_globally: None,
            is_enabled_to_user: None,
            name: None,
            packages: Vec::new(),
            percent_loaded: None,
            priority: None,
            user_pending: None,
            version_id: None,
        }
    }


    /// Sets the value of CustomData
    pub fn set_custom_data(&mut self, value: String) {
        self.custom_data = Some(value);
    }

    /// Gets the value of CustomData
    pub fn get_custom_data(&self) -> Option<&String> {
        self.custom_data.as_ref()
    }

    /// Sets the value of GlobalPending
    pub fn set_global_pending(&mut self, value: bool) {
        self.global_pending = Some(value);
    }

    /// Gets the value of GlobalPending
    pub fn get_global_pending(&self) -> Option<&bool> {
        self.global_pending.as_ref()
    }

    /// Sets the value of GroupId
    pub fn set_group_id(&mut self, value: String) {
        self.group_id = Some(value);
    }

    /// Gets the value of GroupId
    pub fn get_group_id(&self) -> Option<&String> {
        self.group_id.as_ref()
    }

    /// Sets the value of InUse
    pub fn set_in_use(&mut self, value: bool) {
        self.in_use = Some(value);
    }

    /// Gets the value of InUse
    pub fn get_in_use(&self) -> Option<&bool> {
        self.in_use.as_ref()
    }

    /// Sets the value of IsEnabledGlobally
    pub fn set_is_enabled_globally(&mut self, value: bool) {
        self.is_enabled_globally = Some(value);
    }

    /// Gets the value of IsEnabledGlobally
    pub fn get_is_enabled_globally(&self) -> Option<&bool> {
        self.is_enabled_globally.as_ref()
    }

    /// Sets the value of IsEnabledToUser
    pub fn set_is_enabled_to_user(&mut self, value: bool) {
        self.is_enabled_to_user = Some(value);
    }

    /// Gets the value of IsEnabledToUser
    pub fn get_is_enabled_to_user(&self) -> Option<&bool> {
        self.is_enabled_to_user.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Packages
    pub fn set_packages(&mut self, value: Vec<String>) {
        self.packages = value;
    }

    /// Gets the value of Packages
    pub fn get_packages(&self) -> &Vec<String> {
        &self.packages
    }

    /// Sets the value of PercentLoaded
    pub fn set_percent_loaded(&mut self, value: u16) {
        self.percent_loaded = Some(value);
    }

    /// Gets the value of PercentLoaded
    pub fn get_percent_loaded(&self) -> Option<&u16> {
        self.percent_loaded.as_ref()
    }

    /// Sets the value of Priority
    pub fn set_priority(&mut self, value: u32) {
        self.priority = Some(value);
    }

    /// Gets the value of Priority
    pub fn get_priority(&self) -> Option<&u32> {
        self.priority.as_ref()
    }

    /// Sets the value of UserPending
    pub fn set_user_pending(&mut self, value: bool) {
        self.user_pending = Some(value);
    }

    /// Gets the value of UserPending
    pub fn get_user_pending(&self) -> Option<&bool> {
        self.user_pending.as_ref()
    }

    /// Sets the value of VersionId
    pub fn set_version_id(&mut self, value: String) {
        self.version_id = Some(value);
    }

    /// Gets the value of VersionId
    pub fn get_version_id(&self) -> Option<&String> {
        self.version_id.as_ref()
    }
}

