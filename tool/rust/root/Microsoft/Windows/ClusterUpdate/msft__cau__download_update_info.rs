// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ClusterUpdate
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CAU_DownloadUpdateInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CAU_DownloadUpdateInfo {
    #[serde(flatten)]
    pub base: MSFT_CAU_ScanUpdateInfo,

/// 
    #[serde(rename = "UpdateErrorCode")]
    pub update_error_code: Option<i32>,

/// 
    #[serde(rename = "UpdateResultCode")]
    pub update_result_code: Option<u32>,

/// 
    #[serde(rename = "UpdateTimestamp")]
    pub update_timestamp: Option<String>,
}

impl MSFT_CAU_DownloadUpdateInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_CAU_ScanUpdateInfo::new(),
            update_error_code: None,
            update_result_code: None,
            update_timestamp: None,
        }
    }


    /// Sets the value of UpdateErrorCode
    pub fn set_update_error_code(&mut self, value: i32) {
        self.update_error_code = Some(value);
    }

    /// Gets the value of UpdateErrorCode
    pub fn get_update_error_code(&self) -> Option<&i32> {
        self.update_error_code.as_ref()
    }

    /// Sets the value of UpdateResultCode
    pub fn set_update_result_code(&mut self, value: u32) {
        self.update_result_code = Some(value);
    }

    /// Gets the value of UpdateResultCode
    pub fn get_update_result_code(&self) -> Option<&u32> {
        self.update_result_code.as_ref()
    }

    /// Sets the value of UpdateTimestamp
    pub fn set_update_timestamp(&mut self, value: String) {
        self.update_timestamp = Some(value);
    }

    /// Gets the value of UpdateTimestamp
    pub fn get_update_timestamp(&self) -> Option<&String> {
        self.update_timestamp.as_ref()
    }
}

