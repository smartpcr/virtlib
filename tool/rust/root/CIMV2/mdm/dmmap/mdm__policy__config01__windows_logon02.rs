// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_WindowsLogon02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_WindowsLogon02 {

/// 
    #[serde(rename = "AllowAutomaticRestartSignOn")]
    pub allow_automatic_restart_sign_on: Option<String>,

/// 
    #[serde(rename = "ConfigAutomaticRestartSignOn")]
    pub config_automatic_restart_sign_on: Option<String>,

/// 
    #[serde(rename = "DisableLockScreenAppNotifications")]
    pub disable_lock_screen_app_notifications: Option<String>,

/// 
    #[serde(rename = "DontDisplayNetworkSelectionUI")]
    pub dont_display_network_selection_ui: Option<String>,

/// 
    #[serde(rename = "EnableFirstLogonAnimation")]
    pub enable_first_logon_animation: Option<i32>,

/// 
    #[serde(rename = "EnumerateLocalUsersOnDomainJoinedComputers")]
    pub enumerate_local_users_on_domain_joined_computers: Option<String>,

/// 
    #[serde(rename = "HideFastUserSwitching")]
    pub hide_fast_user_switching: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_Config01_WindowsLogon02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_automatic_restart_sign_on: None,
            config_automatic_restart_sign_on: None,
            disable_lock_screen_app_notifications: None,
            dont_display_network_selection_ui: None,
            enable_first_logon_animation: None,
            enumerate_local_users_on_domain_joined_computers: None,
            hide_fast_user_switching: None,
            instance_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of AllowAutomaticRestartSignOn
    pub fn set_allow_automatic_restart_sign_on(&mut self, value: String) {
        self.allow_automatic_restart_sign_on = Some(value);
    }

    /// Gets the value of AllowAutomaticRestartSignOn
    pub fn get_allow_automatic_restart_sign_on(&self) -> Option<&String> {
        self.allow_automatic_restart_sign_on.as_ref()
    }

    /// Sets the value of ConfigAutomaticRestartSignOn
    pub fn set_config_automatic_restart_sign_on(&mut self, value: String) {
        self.config_automatic_restart_sign_on = Some(value);
    }

    /// Gets the value of ConfigAutomaticRestartSignOn
    pub fn get_config_automatic_restart_sign_on(&self) -> Option<&String> {
        self.config_automatic_restart_sign_on.as_ref()
    }

    /// Sets the value of DisableLockScreenAppNotifications
    pub fn set_disable_lock_screen_app_notifications(&mut self, value: String) {
        self.disable_lock_screen_app_notifications = Some(value);
    }

    /// Gets the value of DisableLockScreenAppNotifications
    pub fn get_disable_lock_screen_app_notifications(&self) -> Option<&String> {
        self.disable_lock_screen_app_notifications.as_ref()
    }

    /// Sets the value of DontDisplayNetworkSelectionUI
    pub fn set_dont_display_network_selection_ui(&mut self, value: String) {
        self.dont_display_network_selection_ui = Some(value);
    }

    /// Gets the value of DontDisplayNetworkSelectionUI
    pub fn get_dont_display_network_selection_ui(&self) -> Option<&String> {
        self.dont_display_network_selection_ui.as_ref()
    }

    /// Sets the value of EnableFirstLogonAnimation
    pub fn set_enable_first_logon_animation(&mut self, value: i32) {
        self.enable_first_logon_animation = Some(value);
    }

    /// Gets the value of EnableFirstLogonAnimation
    pub fn get_enable_first_logon_animation(&self) -> Option<&i32> {
        self.enable_first_logon_animation.as_ref()
    }

    /// Sets the value of EnumerateLocalUsersOnDomainJoinedComputers
    pub fn set_enumerate_local_users_on_domain_joined_computers(&mut self, value: String) {
        self.enumerate_local_users_on_domain_joined_computers = Some(value);
    }

    /// Gets the value of EnumerateLocalUsersOnDomainJoinedComputers
    pub fn get_enumerate_local_users_on_domain_joined_computers(&self) -> Option<&String> {
        self.enumerate_local_users_on_domain_joined_computers.as_ref()
    }

    /// Sets the value of HideFastUserSwitching
    pub fn set_hide_fast_user_switching(&mut self, value: i32) {
        self.hide_fast_user_switching = Some(value);
    }

    /// Gets the value of HideFastUserSwitching
    pub fn get_hide_fast_user_switching(&self) -> Option<&i32> {
        self.hide_fast_user_switching.as_ref()
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

