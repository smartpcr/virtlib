// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_FolderRedirectionHealthConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_FolderRedirectionHealthConfiguration {

/// Cautious threshold, in hours. If the number of hours since the last attempted synchronization is greater than or equal to this threshold, the HealthStatus property of the Win32_FolderRedirectionHealth class is set to Caution.
    #[serde(rename = "LastSyncDurationCautionInHours")]
    pub last_sync_duration_caution_in_hours: Option<u32>,

/// Unhealthy threshold, in hours. If the number of hours since the last attempted synchronization is greater than or equal to this threshold, the HealthStatus property of the Win32_FolderRedirectionHealth class is set to Unhealthy.
    #[serde(rename = "LastSyncDurationUnhealthyInHours")]
    pub last_sync_duration_unhealthy_in_hours: Option<u32>,
}

impl Win32_FolderRedirectionHealthConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            last_sync_duration_caution_in_hours: None,
            last_sync_duration_unhealthy_in_hours: None,
        }
    }


    /// Sets the value of LastSyncDurationCautionInHours
    pub fn set_last_sync_duration_caution_in_hours(&mut self, value: u32) {
        self.last_sync_duration_caution_in_hours = Some(value);
    }

    /// Gets the value of LastSyncDurationCautionInHours
    pub fn get_last_sync_duration_caution_in_hours(&self) -> Option<&u32> {
        self.last_sync_duration_caution_in_hours.as_ref()
    }

    /// Sets the value of LastSyncDurationUnhealthyInHours
    pub fn set_last_sync_duration_unhealthy_in_hours(&mut self, value: u32) {
        self.last_sync_duration_unhealthy_in_hours = Some(value);
    }

    /// Gets the value of LastSyncDurationUnhealthyInHours
    pub fn get_last_sync_duration_unhealthy_in_hours(&self) -> Option<&u32> {
        self.last_sync_duration_unhealthy_in_hours.as_ref()
    }
}

