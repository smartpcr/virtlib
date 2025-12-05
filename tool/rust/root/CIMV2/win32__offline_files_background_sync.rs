// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_OfflineFilesBackgroundSync struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_OfflineFilesBackgroundSync {

/// 
    #[serde(rename = "BackgroundSyncWorkOfflineSharesEnabled")]
    pub background_sync_work_offline_shares_enabled: Option<bool>,

/// 
    #[serde(rename = "BlockOutDurationMin")]
    pub block_out_duration_min: Option<u16>,

/// 
    #[serde(rename = "BlockOutStartTimeHoursMinutes")]
    pub block_out_start_time_hours_minutes: Option<u16>,

/// 
    #[serde(rename = "MaxTimeBetweenSyncs")]
    pub max_time_between_syncs: Option<u16>,

/// 
    #[serde(rename = "SyncInterval")]
    pub sync_interval: Option<u16>,

/// 
    #[serde(rename = "SyncVariance")]
    pub sync_variance: Option<u16>,
}

impl Win32_OfflineFilesBackgroundSync {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            background_sync_work_offline_shares_enabled: None,
            block_out_duration_min: None,
            block_out_start_time_hours_minutes: None,
            max_time_between_syncs: None,
            sync_interval: None,
            sync_variance: None,
        }
    }


    /// Sets the value of BackgroundSyncWorkOfflineSharesEnabled
    pub fn set_background_sync_work_offline_shares_enabled(&mut self, value: bool) {
        self.background_sync_work_offline_shares_enabled = Some(value);
    }

    /// Gets the value of BackgroundSyncWorkOfflineSharesEnabled
    pub fn get_background_sync_work_offline_shares_enabled(&self) -> Option<&bool> {
        self.background_sync_work_offline_shares_enabled.as_ref()
    }

    /// Sets the value of BlockOutDurationMin
    pub fn set_block_out_duration_min(&mut self, value: u16) {
        self.block_out_duration_min = Some(value);
    }

    /// Gets the value of BlockOutDurationMin
    pub fn get_block_out_duration_min(&self) -> Option<&u16> {
        self.block_out_duration_min.as_ref()
    }

    /// Sets the value of BlockOutStartTimeHoursMinutes
    pub fn set_block_out_start_time_hours_minutes(&mut self, value: u16) {
        self.block_out_start_time_hours_minutes = Some(value);
    }

    /// Gets the value of BlockOutStartTimeHoursMinutes
    pub fn get_block_out_start_time_hours_minutes(&self) -> Option<&u16> {
        self.block_out_start_time_hours_minutes.as_ref()
    }

    /// Sets the value of MaxTimeBetweenSyncs
    pub fn set_max_time_between_syncs(&mut self, value: u16) {
        self.max_time_between_syncs = Some(value);
    }

    /// Gets the value of MaxTimeBetweenSyncs
    pub fn get_max_time_between_syncs(&self) -> Option<&u16> {
        self.max_time_between_syncs.as_ref()
    }

    /// Sets the value of SyncInterval
    pub fn set_sync_interval(&mut self, value: u16) {
        self.sync_interval = Some(value);
    }

    /// Gets the value of SyncInterval
    pub fn get_sync_interval(&self) -> Option<&u16> {
        self.sync_interval.as_ref()
    }

    /// Sets the value of SyncVariance
    pub fn set_sync_variance(&mut self, value: u16) {
        self.sync_variance = Some(value);
    }

    /// Gets the value of SyncVariance
    pub fn get_sync_variance(&self) -> Option<&u16> {
        self.sync_variance.as_ref()
    }
}

