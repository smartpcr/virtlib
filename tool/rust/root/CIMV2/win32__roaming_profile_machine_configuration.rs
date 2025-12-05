// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_RoamingProfileMachineConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_RoamingProfileMachineConfiguration {

/// If true, add the Administrator group to roaming user profiles.
    #[serde(rename = "AddAdminGroupToRUPEnabled")]
    pub add_admin_group_to_rupenabled: Option<bool>,

/// If true, allow cross-forest user policy and roaming user profiles. If false, a roaming profile user receives a local profile when logged on to a cross-forest domain.
    #[serde(rename = "AllowCrossForestUserPolicy")]
    pub allow_cross_forest_user_policy: Option<bool>,

/// Contains the parameter for the background upload of a roaming user profile's registry file while the user is logged on.
    #[serde(rename = "BackgroundUploadParams")]
    pub background_upload_params: Option<Win32_RoamingProfileBackgroundUploadParams>,

/// If the DeleteRoamingCache property is true, this property specifies the number of days after which a user profile should be deleted. User profiles older than this number of days are deleted when the computer is restarted.
    #[serde(rename = "DeleteProfilesOlderDays")]
    pub delete_profiles_older_days: Option<u16>,

/// If true, cached copies of the roaming profile are deleted at log off
    #[serde(rename = "DeleteRoamingCacheEnabled")]
    pub delete_roaming_cache_enabled: Option<bool>,

/// If true, do not detect slow network connections. If false, use the SlowLinkTimeOutParams property to determine whether the computer has a slow network connection.
    #[serde(rename = "DetectSlowLinkDisabled")]
    pub detect_slow_link_disabled: Option<bool>,

/// If true, do not forcibly unload the user's registry when the user logs off.
    #[serde(rename = "ForceUnloadDisabled")]
    pub force_unload_disabled: Option<bool>,

/// Indicates if the settings configured through this WMI class are taking affect.
    #[serde(rename = "IsConfiguredByWMI")]
    pub is_configured_by_wmi: Option<bool>,

/// The roaming profile path to be set for all users that log on to this computer. The path should be in the form of \\ComputerName\ShareName\%USERNAME%.
    #[serde(rename = "MachineProfilePath")]
    pub machine_profile_path: Option<String>,

/// If true, allow only local user profiles.
    #[serde(rename = "OnlyAllowLocalProfiles")]
    pub only_allow_local_profiles: Option<bool>,

/// If true, don't check the owners of user profiles.
    #[serde(rename = "OwnerCheckDisabled")]
    pub owner_check_disabled: Option<bool>,

/// If true, a configured roaming profile will only be downloaded if the machine is a primary computer for the user.
    #[serde(rename = "PrimaryComputerEnabled")]
    pub primary_computer_enabled: Option<bool>,

/// If true, prevent roaming profile changes from being copied to the server.
    #[serde(rename = "ProfileUploadDisabled")]
    pub profile_upload_disabled: Option<bool>,

/// Contains slow network connection timeout parameters to be used for user profiles.
    #[serde(rename = "SlowLinkTimeOutParams")]
    pub slow_link_time_out_params: Option<Win32_RoamingProfileSlowLinkParams>,

/// If true, the user is prompted to specify whether his or her profile should be downloaded even when the network connection is slow.
    #[serde(rename = "SlowLinkUIEnabled")]
    pub slow_link_uienabled: Option<bool>,

/// If true, do not allow users to log in with temporary profiles.
    #[serde(rename = "TempProfileLogonBlocked")]
    pub temp_profile_logon_blocked: Option<bool>,

/// The maximum time, in seconds, to wait for the network transport to be available if a user has a roaming user profile. If the network is unavailable after this time has elapsed, the user is logged on, but the profile is not synchronized.
    #[serde(rename = "WaitForNetworkInSec")]
    pub wait_for_network_in_sec: Option<u16>,

/// If true, wait for a remote user profile.
    #[serde(rename = "WaitForRemoteProfile")]
    pub wait_for_remote_profile: Option<bool>,
}

impl Win32_RoamingProfileMachineConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            add_admin_group_to_rupenabled: None,
            allow_cross_forest_user_policy: None,
            background_upload_params: None,
            delete_profiles_older_days: None,
            delete_roaming_cache_enabled: None,
            detect_slow_link_disabled: None,
            force_unload_disabled: None,
            is_configured_by_wmi: None,
            machine_profile_path: None,
            only_allow_local_profiles: None,
            owner_check_disabled: None,
            primary_computer_enabled: None,
            profile_upload_disabled: None,
            slow_link_time_out_params: None,
            slow_link_uienabled: None,
            temp_profile_logon_blocked: None,
            wait_for_network_in_sec: None,
            wait_for_remote_profile: None,
        }
    }


    /// Sets the value of AddAdminGroupToRUPEnabled
    pub fn set_add_admin_group_to_rupenabled(&mut self, value: bool) {
        self.add_admin_group_to_rupenabled = Some(value);
    }

    /// Gets the value of AddAdminGroupToRUPEnabled
    pub fn get_add_admin_group_to_rupenabled(&self) -> Option<&bool> {
        self.add_admin_group_to_rupenabled.as_ref()
    }

    /// Sets the value of AllowCrossForestUserPolicy
    pub fn set_allow_cross_forest_user_policy(&mut self, value: bool) {
        self.allow_cross_forest_user_policy = Some(value);
    }

    /// Gets the value of AllowCrossForestUserPolicy
    pub fn get_allow_cross_forest_user_policy(&self) -> Option<&bool> {
        self.allow_cross_forest_user_policy.as_ref()
    }

    /// Sets the value of BackgroundUploadParams
    pub fn set_background_upload_params(&mut self, value: Win32_RoamingProfileBackgroundUploadParams) {
        self.background_upload_params = Some(value);
    }

    /// Gets the value of BackgroundUploadParams
    pub fn get_background_upload_params(&self) -> Option<&Win32_RoamingProfileBackgroundUploadParams> {
        self.background_upload_params.as_ref()
    }

    /// Sets the value of DeleteProfilesOlderDays
    pub fn set_delete_profiles_older_days(&mut self, value: u16) {
        self.delete_profiles_older_days = Some(value);
    }

    /// Gets the value of DeleteProfilesOlderDays
    pub fn get_delete_profiles_older_days(&self) -> Option<&u16> {
        self.delete_profiles_older_days.as_ref()
    }

    /// Sets the value of DeleteRoamingCacheEnabled
    pub fn set_delete_roaming_cache_enabled(&mut self, value: bool) {
        self.delete_roaming_cache_enabled = Some(value);
    }

    /// Gets the value of DeleteRoamingCacheEnabled
    pub fn get_delete_roaming_cache_enabled(&self) -> Option<&bool> {
        self.delete_roaming_cache_enabled.as_ref()
    }

    /// Sets the value of DetectSlowLinkDisabled
    pub fn set_detect_slow_link_disabled(&mut self, value: bool) {
        self.detect_slow_link_disabled = Some(value);
    }

    /// Gets the value of DetectSlowLinkDisabled
    pub fn get_detect_slow_link_disabled(&self) -> Option<&bool> {
        self.detect_slow_link_disabled.as_ref()
    }

    /// Sets the value of ForceUnloadDisabled
    pub fn set_force_unload_disabled(&mut self, value: bool) {
        self.force_unload_disabled = Some(value);
    }

    /// Gets the value of ForceUnloadDisabled
    pub fn get_force_unload_disabled(&self) -> Option<&bool> {
        self.force_unload_disabled.as_ref()
    }

    /// Sets the value of IsConfiguredByWMI
    pub fn set_is_configured_by_wmi(&mut self, value: bool) {
        self.is_configured_by_wmi = Some(value);
    }

    /// Gets the value of IsConfiguredByWMI
    pub fn get_is_configured_by_wmi(&self) -> Option<&bool> {
        self.is_configured_by_wmi.as_ref()
    }

    /// Sets the value of MachineProfilePath
    pub fn set_machine_profile_path(&mut self, value: String) {
        self.machine_profile_path = Some(value);
    }

    /// Gets the value of MachineProfilePath
    pub fn get_machine_profile_path(&self) -> Option<&String> {
        self.machine_profile_path.as_ref()
    }

    /// Sets the value of OnlyAllowLocalProfiles
    pub fn set_only_allow_local_profiles(&mut self, value: bool) {
        self.only_allow_local_profiles = Some(value);
    }

    /// Gets the value of OnlyAllowLocalProfiles
    pub fn get_only_allow_local_profiles(&self) -> Option<&bool> {
        self.only_allow_local_profiles.as_ref()
    }

    /// Sets the value of OwnerCheckDisabled
    pub fn set_owner_check_disabled(&mut self, value: bool) {
        self.owner_check_disabled = Some(value);
    }

    /// Gets the value of OwnerCheckDisabled
    pub fn get_owner_check_disabled(&self) -> Option<&bool> {
        self.owner_check_disabled.as_ref()
    }

    /// Sets the value of PrimaryComputerEnabled
    pub fn set_primary_computer_enabled(&mut self, value: bool) {
        self.primary_computer_enabled = Some(value);
    }

    /// Gets the value of PrimaryComputerEnabled
    pub fn get_primary_computer_enabled(&self) -> Option<&bool> {
        self.primary_computer_enabled.as_ref()
    }

    /// Sets the value of ProfileUploadDisabled
    pub fn set_profile_upload_disabled(&mut self, value: bool) {
        self.profile_upload_disabled = Some(value);
    }

    /// Gets the value of ProfileUploadDisabled
    pub fn get_profile_upload_disabled(&self) -> Option<&bool> {
        self.profile_upload_disabled.as_ref()
    }

    /// Sets the value of SlowLinkTimeOutParams
    pub fn set_slow_link_time_out_params(&mut self, value: Win32_RoamingProfileSlowLinkParams) {
        self.slow_link_time_out_params = Some(value);
    }

    /// Gets the value of SlowLinkTimeOutParams
    pub fn get_slow_link_time_out_params(&self) -> Option<&Win32_RoamingProfileSlowLinkParams> {
        self.slow_link_time_out_params.as_ref()
    }

    /// Sets the value of SlowLinkUIEnabled
    pub fn set_slow_link_uienabled(&mut self, value: bool) {
        self.slow_link_uienabled = Some(value);
    }

    /// Gets the value of SlowLinkUIEnabled
    pub fn get_slow_link_uienabled(&self) -> Option<&bool> {
        self.slow_link_uienabled.as_ref()
    }

    /// Sets the value of TempProfileLogonBlocked
    pub fn set_temp_profile_logon_blocked(&mut self, value: bool) {
        self.temp_profile_logon_blocked = Some(value);
    }

    /// Gets the value of TempProfileLogonBlocked
    pub fn get_temp_profile_logon_blocked(&self) -> Option<&bool> {
        self.temp_profile_logon_blocked.as_ref()
    }

    /// Sets the value of WaitForNetworkInSec
    pub fn set_wait_for_network_in_sec(&mut self, value: u16) {
        self.wait_for_network_in_sec = Some(value);
    }

    /// Gets the value of WaitForNetworkInSec
    pub fn get_wait_for_network_in_sec(&self) -> Option<&u16> {
        self.wait_for_network_in_sec.as_ref()
    }

    /// Sets the value of WaitForRemoteProfile
    pub fn set_wait_for_remote_profile(&mut self, value: bool) {
        self.wait_for_remote_profile = Some(value);
    }

    /// Gets the value of WaitForRemoteProfile
    pub fn get_wait_for_remote_profile(&self) -> Option<&bool> {
        self.wait_for_remote_profile.as_ref()
    }
}

