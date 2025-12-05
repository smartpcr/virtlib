// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_UserStateConfigurationControls struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_UserStateConfigurationControls {

/// Controls whether the computer's folder redirection feature settings are configured by using UST Manageability WMI classes or by using Group Policy.
    #[serde(rename = "FolderRedirection")]
    pub folder_redirection: Option<UserStateConfigurationControls_FolderRedirection>,

/// Controls whether the computer's Offline files feature settings are configured by using UST Manageability WMI classes or by using Group Policy.
    #[serde(rename = "OfflineFiles")]
    pub offline_files: Option<UserStateConfigurationControls_OfflineFiles>,

/// Controls whether the computer's roaming user profile feature settings are configured by using UST Manageability WMI classes or by using Group Policy.
    #[serde(rename = "RoamingUserProfile")]
    pub roaming_user_profile: Option<UserStateConfigurationControls_RoamingUserProfile>,
}

impl Win32_UserStateConfigurationControls {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            folder_redirection: None,
            offline_files: None,
            roaming_user_profile: None,
        }
    }


    /// Sets the value of FolderRedirection
    pub fn set_folder_redirection(&mut self, value: UserStateConfigurationControls_FolderRedirection) {
        self.folder_redirection = Some(value);
    }

    /// Gets the value of FolderRedirection
    pub fn get_folder_redirection(&self) -> Option<&UserStateConfigurationControls_FolderRedirection> {
        self.folder_redirection.as_ref()
    }

    /// Sets the value of OfflineFiles
    pub fn set_offline_files(&mut self, value: UserStateConfigurationControls_OfflineFiles) {
        self.offline_files = Some(value);
    }

    /// Gets the value of OfflineFiles
    pub fn get_offline_files(&self) -> Option<&UserStateConfigurationControls_OfflineFiles> {
        self.offline_files.as_ref()
    }

    /// Sets the value of RoamingUserProfile
    pub fn set_roaming_user_profile(&mut self, value: UserStateConfigurationControls_RoamingUserProfile) {
        self.roaming_user_profile = Some(value);
    }

    /// Gets the value of RoamingUserProfile
    pub fn get_roaming_user_profile(&self) -> Option<&UserStateConfigurationControls_RoamingUserProfile> {
        self.roaming_user_profile.as_ref()
    }
}

