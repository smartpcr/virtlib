// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_OfflineFilesDiskSpaceLimit struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_OfflineFilesDiskSpaceLimit {

/// 
    #[serde(rename = "AutoCacheSizeInMB")]
    pub auto_cache_size_in_mb: Option<u32>,

/// 
    #[serde(rename = "TotalSizeInMB")]
    pub total_size_in_mb: Option<u32>,
}

impl Win32_OfflineFilesDiskSpaceLimit {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            auto_cache_size_in_mb: None,
            total_size_in_mb: None,
        }
    }


    /// Sets the value of AutoCacheSizeInMB
    pub fn set_auto_cache_size_in_mb(&mut self, value: u32) {
        self.auto_cache_size_in_mb = Some(value);
    }

    /// Gets the value of AutoCacheSizeInMB
    pub fn get_auto_cache_size_in_mb(&self) -> Option<&u32> {
        self.auto_cache_size_in_mb.as_ref()
    }

    /// Sets the value of TotalSizeInMB
    pub fn set_total_size_in_mb(&mut self, value: u32) {
        self.total_size_in_mb = Some(value);
    }

    /// Gets the value of TotalSizeInMB
    pub fn get_total_size_in_mb(&self) -> Option<&u32> {
        self.total_size_in_mb.as_ref()
    }
}

