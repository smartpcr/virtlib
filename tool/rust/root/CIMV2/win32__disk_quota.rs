// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_DiskQuota struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_DiskQuota {

/// 
    #[serde(rename = "DiskSpaceUsed")]
    pub disk_space_used: Option<u64>,

/// 
    #[serde(rename = "Limit")]
    pub limit: Option<u64>,

/// 
    #[serde(rename = "QuotaVolume")]
    pub quota_volume: Option<Win32_LogicalDisk>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u32>,

/// 
    #[serde(rename = "User")]
    pub user: Option<Win32_Account>,

/// 
    #[serde(rename = "WarningLimit")]
    pub warning_limit: Option<u64>,
}

impl Win32_DiskQuota {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            disk_space_used: None,
            limit: None,
            quota_volume: None,
            status: None,
            user: None,
            warning_limit: None,
        }
    }


    /// Sets the value of DiskSpaceUsed
    pub fn set_disk_space_used(&mut self, value: u64) {
        self.disk_space_used = Some(value);
    }

    /// Gets the value of DiskSpaceUsed
    pub fn get_disk_space_used(&self) -> Option<&u64> {
        self.disk_space_used.as_ref()
    }

    /// Sets the value of Limit
    pub fn set_limit(&mut self, value: u64) {
        self.limit = Some(value);
    }

    /// Gets the value of Limit
    pub fn get_limit(&self) -> Option<&u64> {
        self.limit.as_ref()
    }

    /// Sets the value of QuotaVolume
    pub fn set_quota_volume(&mut self, value: Win32_LogicalDisk) {
        self.quota_volume = Some(value);
    }

    /// Gets the value of QuotaVolume
    pub fn get_quota_volume(&self) -> Option<&Win32_LogicalDisk> {
        self.quota_volume.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u32> {
        self.status.as_ref()
    }

    /// Sets the value of User
    pub fn set_user(&mut self, value: Win32_Account) {
        self.user = Some(value);
    }

    /// Gets the value of User
    pub fn get_user(&self) -> Option<&Win32_Account> {
        self.user.as_ref()
    }

    /// Sets the value of WarningLimit
    pub fn set_warning_limit(&mut self, value: u64) {
        self.warning_limit = Some(value);
    }

    /// Gets the value of WarningLimit
    pub fn get_warning_limit(&self) -> Option<&u64> {
        self.warning_limit.as_ref()
    }
}

