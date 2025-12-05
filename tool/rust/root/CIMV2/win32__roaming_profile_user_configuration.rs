// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_RoamingProfileUserConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_RoamingProfileUserConfiguration {

/// An array of strings containing network directories to synchronize at when the user logs on to or off of a local computer.
    #[serde(rename = "DirectoriesToSyncAtLogonLogoff")]
    pub directories_to_sync_at_logon_logoff: Vec<String>,

/// An array of strings containing directories to exclude from the roaming profile.
    #[serde(rename = "ExcludedProfileDirs")]
    pub excluded_profile_dirs: Vec<String>,

/// Indicates if the settings configured through this WMI class are taking affect.
    #[serde(rename = "IsConfiguredByWMI")]
    pub is_configured_by_wmi: Option<bool>,
}

impl Win32_RoamingProfileUserConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            directories_to_sync_at_logon_logoff: Vec::new(),
            excluded_profile_dirs: Vec::new(),
            is_configured_by_wmi: None,
        }
    }


    /// Sets the value of DirectoriesToSyncAtLogonLogoff
    pub fn set_directories_to_sync_at_logon_logoff(&mut self, value: Vec<String>) {
        self.directories_to_sync_at_logon_logoff = value;
    }

    /// Gets the value of DirectoriesToSyncAtLogonLogoff
    pub fn get_directories_to_sync_at_logon_logoff(&self) -> &Vec<String> {
        &self.directories_to_sync_at_logon_logoff
    }

    /// Sets the value of ExcludedProfileDirs
    pub fn set_excluded_profile_dirs(&mut self, value: Vec<String>) {
        self.excluded_profile_dirs = value;
    }

    /// Gets the value of ExcludedProfileDirs
    pub fn get_excluded_profile_dirs(&self) -> &Vec<String> {
        &self.excluded_profile_dirs
    }

    /// Sets the value of IsConfiguredByWMI
    pub fn set_is_configured_by_wmi(&mut self, value: bool) {
        self.is_configured_by_wmi = Some(value);
    }

    /// Gets the value of IsConfiguredByWMI
    pub fn get_is_configured_by_wmi(&self) -> Option<&bool> {
        self.is_configured_by_wmi.as_ref()
    }
}

