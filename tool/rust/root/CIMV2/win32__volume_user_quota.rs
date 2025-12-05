// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_VolumeUserQuota struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_VolumeUserQuota {

/// 
    #[serde(rename = "Account")]
    pub account: Option<Win32_Account>,

/// 
    #[serde(rename = "DiskSpaceUsed")]
    pub disk_space_used: Option<u64>,

/// 
    #[serde(rename = "Limit")]
    pub limit: Option<u64>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u32>,

/// 
    #[serde(rename = "Volume")]
    pub volume: Option<Win32_Volume>,

/// 
    #[serde(rename = "WarningLimit")]
    pub warning_limit: Option<u64>,
}

impl Win32_VolumeUserQuota {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            account: None,
            disk_space_used: None,
            limit: None,
            status: None,
            volume: None,
            warning_limit: None,
        }
    }


    /// Sets the value of Account
    pub fn set_account(&mut self, value: Win32_Account) {
        self.account = Some(value);
    }

    /// Gets the value of Account
    pub fn get_account(&self) -> Option<&Win32_Account> {
        self.account.as_ref()
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

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u32> {
        self.status.as_ref()
    }

    /// Sets the value of Volume
    pub fn set_volume(&mut self, value: Win32_Volume) {
        self.volume = Some(value);
    }

    /// Gets the value of Volume
    pub fn get_volume(&self) -> Option<&Win32_Volume> {
        self.volume.as_ref()
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

