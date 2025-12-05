// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_OfflineFilesHealth struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_OfflineFilesHealth {

/// A DATETIME value, in string format, that represents the last time this folder was successfully synchronized to the Offline Files cache.
    #[serde(rename = "LastSuccessfulSyncTime")]
    pub last_successful_sync_time: Option<String>,

/// The status of the last attempt to synchronize this folder to the Offline Files cache.
    #[serde(rename = "LastSyncStatus")]
    pub last_sync_status: Option<u8>,

/// A DATETIME value, in string format, that represents the last time an attempt was made to synchronized this folder to the Offline Files cache, even if it was unsuccessful.
    #[serde(rename = "LastSyncTime")]
    pub last_sync_time: Option<String>,

/// If true, the Offline Files feature is enabled for this folder.
    #[serde(rename = "OfflineAccessEnabled")]
    pub offline_access_enabled: Option<bool>,

/// If true, the share is working in Online mode
    #[serde(rename = "OnlineMode")]
    pub online_mode: Option<bool>,
}

impl Win32_OfflineFilesHealth {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            last_successful_sync_time: None,
            last_sync_status: None,
            last_sync_time: None,
            offline_access_enabled: None,
            online_mode: None,
        }
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
    pub fn set_last_sync_status(&mut self, value: u8) {
        self.last_sync_status = Some(value);
    }

    /// Gets the value of LastSyncStatus
    pub fn get_last_sync_status(&self) -> Option<&u8> {
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

    /// Sets the value of OnlineMode
    pub fn set_online_mode(&mut self, value: bool) {
        self.online_mode = Some(value);
    }

    /// Gets the value of OnlineMode
    pub fn get_online_mode(&self) -> Option<&bool> {
        self.online_mode.as_ref()
    }
}

