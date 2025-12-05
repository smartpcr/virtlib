// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_FolderRedirectionHealth struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_FolderRedirectionHealth {

/// The health status of this folder, based on the values that were set in the Win32_FolderRedirectionHealthConfiguration properties.
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<FolderRedirectionHealth_HealthStatus>,

/// The last time this folder was successfully synchronized to the Offline Files cache.
    #[serde(rename = "LastSuccessfulSyncTime")]
    pub last_successful_sync_time: Option<String>,

/// The status of the last attempt to synchronize this folder to the Offline Files cache.
    #[serde(rename = "LastSyncStatus")]
    pub last_sync_status: Option<FolderRedirectionHealth_LastSyncStatus>,

/// The last time an attempt was made to synchronized this folder to the Offline Files cache, even if it was unsuccessful.
    #[serde(rename = "LastSyncTime")]
    pub last_sync_time: Option<String>,

/// If true, the Offline Files feature is enabled for this folder.
    #[serde(rename = "OfflineAccessEnabled")]
    pub offline_access_enabled: Option<bool>,

/// known folder unique id (guid)
    #[serde(rename = "OfflineFileNameFolderGUID")]
    pub offline_file_name_folder_guid: Option<String>,

/// If true, indicate if this folder is being redirected.
    #[serde(rename = "Redirected")]
    pub redirected: Option<bool>,
}

impl Win32_FolderRedirectionHealth {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            health_status: None,
            last_successful_sync_time: None,
            last_sync_status: None,
            last_sync_time: None,
            offline_access_enabled: None,
            offline_file_name_folder_guid: None,
            redirected: None,
        }
    }


    /// Sets the value of HealthStatus
    pub fn set_health_status(&mut self, value: FolderRedirectionHealth_HealthStatus) {
        self.health_status = Some(value);
    }

    /// Gets the value of HealthStatus
    pub fn get_health_status(&self) -> Option<&FolderRedirectionHealth_HealthStatus> {
        self.health_status.as_ref()
    }

    /// Sets the value of LastSuccessfulSyncTime
    pub fn set_last_successful_sync_time(&mut self, value: String) {
        self.last_successful_sync_time = Some(value);
    }

    /// Gets the value of LastSuccessfulSyncTime
    pub fn get_last_successful_sync_time(&self) -> Option<&String> {
        self.last_successful_sync_time.as_ref()
    }

    /// Sets the value of LastSyncStatus
    pub fn set_last_sync_status(&mut self, value: FolderRedirectionHealth_LastSyncStatus) {
        self.last_sync_status = Some(value);
    }

    /// Gets the value of LastSyncStatus
    pub fn get_last_sync_status(&self) -> Option<&FolderRedirectionHealth_LastSyncStatus> {
        self.last_sync_status.as_ref()
    }

    /// Sets the value of LastSyncTime
    pub fn set_last_sync_time(&mut self, value: String) {
        self.last_sync_time = Some(value);
    }

    /// Gets the value of LastSyncTime
    pub fn get_last_sync_time(&self) -> Option<&String> {
        self.last_sync_time.as_ref()
    }

    /// Sets the value of OfflineAccessEnabled
    pub fn set_offline_access_enabled(&mut self, value: bool) {
        self.offline_access_enabled = Some(value);
    }

    /// Gets the value of OfflineAccessEnabled
    pub fn get_offline_access_enabled(&self) -> Option<&bool> {
        self.offline_access_enabled.as_ref()
    }

    /// Sets the value of OfflineFileNameFolderGUID
    pub fn set_offline_file_name_folder_guid(&mut self, value: String) {
        self.offline_file_name_folder_guid = Some(value);
    }

    /// Gets the value of OfflineFileNameFolderGUID
    pub fn get_offline_file_name_folder_guid(&self) -> Option<&String> {
        self.offline_file_name_folder_guid.as_ref()
    }

    /// Sets the value of Redirected
    pub fn set_redirected(&mut self, value: bool) {
        self.redirected = Some(value);
    }

    /// Gets the value of Redirected
    pub fn get_redirected(&self) -> Option<&bool> {
        self.redirected.as_ref()
    }
}

