// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_ApplicationManagement02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_ApplicationManagement02 {

/// 
    #[serde(rename = "AllowAllTrustedApps")]
    pub allow_all_trusted_apps: Option<i32>,

/// 
    #[serde(rename = "AllowAppStoreAutoUpdate")]
    pub allow_app_store_auto_update: Option<i32>,

/// 
    #[serde(rename = "AllowAutomaticAppArchiving")]
    pub allow_automatic_app_archiving: Option<i32>,

/// 
    #[serde(rename = "AllowDeveloperUnlock")]
    pub allow_developer_unlock: Option<i32>,

/// 
    #[serde(rename = "AllowGameDVR")]
    pub allow_game_dvr: Option<i32>,

/// 
    #[serde(rename = "AllowSharedUserAppData")]
    pub allow_shared_user_app_data: Option<i32>,

/// 
    #[serde(rename = "BlockNonAdminUserInstall")]
    pub block_non_admin_user_install: Option<i32>,

/// 
    #[serde(rename = "DisableStoreOriginatedApps")]
    pub disable_store_originated_apps: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LaunchAppAfterLogOn")]
    pub launch_app_after_log_on: Option<String>,

/// 
    #[serde(rename = "MSIAllowUserControlOverInstall")]
    pub msiallow_user_control_over_install: Option<i32>,

/// 
    #[serde(rename = "MSIAlwaysInstallWithElevatedPrivileges")]
    pub msialways_install_with_elevated_privileges: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RequirePrivateStoreOnly")]
    pub require_private_store_only: Option<i32>,

/// 
    #[serde(rename = "RestrictAppDataToSystemVolume")]
    pub restrict_app_data_to_system_volume: Option<i32>,

/// 
    #[serde(rename = "RestrictAppToSystemVolume")]
    pub restrict_app_to_system_volume: Option<i32>,

/// 
    #[serde(rename = "ScheduleForceRestartForUpdateFailures")]
    pub schedule_force_restart_for_update_failures: Option<String>,
}

impl MDM_Policy_Config01_ApplicationManagement02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_all_trusted_apps: None,
            allow_app_store_auto_update: None,
            allow_automatic_app_archiving: None,
            allow_developer_unlock: None,
            allow_game_dvr: None,
            allow_shared_user_app_data: None,
            block_non_admin_user_install: None,
            disable_store_originated_apps: None,
            instance_id: None,
            launch_app_after_log_on: None,
            msiallow_user_control_over_install: None,
            msialways_install_with_elevated_privileges: None,
            parent_id: None,
            require_private_store_only: None,
            restrict_app_data_to_system_volume: None,
            restrict_app_to_system_volume: None,
            schedule_force_restart_for_update_failures: None,
        }
    }


    /// Sets the value of AllowAllTrustedApps
    pub fn set_allow_all_trusted_apps(&mut self, value: i32) {
        self.allow_all_trusted_apps = Some(value);
    }

    /// Gets the value of AllowAllTrustedApps
    pub fn get_allow_all_trusted_apps(&self) -> Option<&i32> {
        self.allow_all_trusted_apps.as_ref()
    }

    /// Sets the value of AllowAppStoreAutoUpdate
    pub fn set_allow_app_store_auto_update(&mut self, value: i32) {
        self.allow_app_store_auto_update = Some(value);
    }

    /// Gets the value of AllowAppStoreAutoUpdate
    pub fn get_allow_app_store_auto_update(&self) -> Option<&i32> {
        self.allow_app_store_auto_update.as_ref()
    }

    /// Sets the value of AllowAutomaticAppArchiving
    pub fn set_allow_automatic_app_archiving(&mut self, value: i32) {
        self.allow_automatic_app_archiving = Some(value);
    }

    /// Gets the value of AllowAutomaticAppArchiving
    pub fn get_allow_automatic_app_archiving(&self) -> Option<&i32> {
        self.allow_automatic_app_archiving.as_ref()
    }

    /// Sets the value of AllowDeveloperUnlock
    pub fn set_allow_developer_unlock(&mut self, value: i32) {
        self.allow_developer_unlock = Some(value);
    }

    /// Gets the value of AllowDeveloperUnlock
    pub fn get_allow_developer_unlock(&self) -> Option<&i32> {
        self.allow_developer_unlock.as_ref()
    }

    /// Sets the value of AllowGameDVR
    pub fn set_allow_game_dvr(&mut self, value: i32) {
        self.allow_game_dvr = Some(value);
    }

    /// Gets the value of AllowGameDVR
    pub fn get_allow_game_dvr(&self) -> Option<&i32> {
        self.allow_game_dvr.as_ref()
    }

    /// Sets the value of AllowSharedUserAppData
    pub fn set_allow_shared_user_app_data(&mut self, value: i32) {
        self.allow_shared_user_app_data = Some(value);
    }

    /// Gets the value of AllowSharedUserAppData
    pub fn get_allow_shared_user_app_data(&self) -> Option<&i32> {
        self.allow_shared_user_app_data.as_ref()
    }

    /// Sets the value of BlockNonAdminUserInstall
    pub fn set_block_non_admin_user_install(&mut self, value: i32) {
        self.block_non_admin_user_install = Some(value);
    }

    /// Gets the value of BlockNonAdminUserInstall
    pub fn get_block_non_admin_user_install(&self) -> Option<&i32> {
        self.block_non_admin_user_install.as_ref()
    }

    /// Sets the value of DisableStoreOriginatedApps
    pub fn set_disable_store_originated_apps(&mut self, value: i32) {
        self.disable_store_originated_apps = Some(value);
    }

    /// Gets the value of DisableStoreOriginatedApps
    pub fn get_disable_store_originated_apps(&self) -> Option<&i32> {
        self.disable_store_originated_apps.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of LaunchAppAfterLogOn
    pub fn set_launch_app_after_log_on(&mut self, value: String) {
        self.launch_app_after_log_on = Some(value);
    }

    /// Gets the value of LaunchAppAfterLogOn
    pub fn get_launch_app_after_log_on(&self) -> Option<&String> {
        self.launch_app_after_log_on.as_ref()
    }

    /// Sets the value of MSIAllowUserControlOverInstall
    pub fn set_msiallow_user_control_over_install(&mut self, value: i32) {
        self.msiallow_user_control_over_install = Some(value);
    }

    /// Gets the value of MSIAllowUserControlOverInstall
    pub fn get_msiallow_user_control_over_install(&self) -> Option<&i32> {
        self.msiallow_user_control_over_install.as_ref()
    }

    /// Sets the value of MSIAlwaysInstallWithElevatedPrivileges
    pub fn set_msialways_install_with_elevated_privileges(&mut self, value: i32) {
        self.msialways_install_with_elevated_privileges = Some(value);
    }

    /// Gets the value of MSIAlwaysInstallWithElevatedPrivileges
    pub fn get_msialways_install_with_elevated_privileges(&self) -> Option<&i32> {
        self.msialways_install_with_elevated_privileges.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of RequirePrivateStoreOnly
    pub fn set_require_private_store_only(&mut self, value: i32) {
        self.require_private_store_only = Some(value);
    }

    /// Gets the value of RequirePrivateStoreOnly
    pub fn get_require_private_store_only(&self) -> Option<&i32> {
        self.require_private_store_only.as_ref()
    }

    /// Sets the value of RestrictAppDataToSystemVolume
    pub fn set_restrict_app_data_to_system_volume(&mut self, value: i32) {
        self.restrict_app_data_to_system_volume = Some(value);
    }

    /// Gets the value of RestrictAppDataToSystemVolume
    pub fn get_restrict_app_data_to_system_volume(&self) -> Option<&i32> {
        self.restrict_app_data_to_system_volume.as_ref()
    }

    /// Sets the value of RestrictAppToSystemVolume
    pub fn set_restrict_app_to_system_volume(&mut self, value: i32) {
        self.restrict_app_to_system_volume = Some(value);
    }

    /// Gets the value of RestrictAppToSystemVolume
    pub fn get_restrict_app_to_system_volume(&self) -> Option<&i32> {
        self.restrict_app_to_system_volume.as_ref()
    }

    /// Sets the value of ScheduleForceRestartForUpdateFailures
    pub fn set_schedule_force_restart_for_update_failures(&mut self, value: String) {
        self.schedule_force_restart_for_update_failures = Some(value);
    }

    /// Gets the value of ScheduleForceRestartForUpdateFailures
    pub fn get_schedule_force_restart_for_update_failures(&self) -> Option<&String> {
        self.schedule_force_restart_for_update_failures.as_ref()
    }
}

