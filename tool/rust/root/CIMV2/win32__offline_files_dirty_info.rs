// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_OfflineFilesDirtyInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_OfflineFilesDirtyInfo {

/// 
    #[serde(rename = "LocalDirtyByteCount")]
    pub local_dirty_byte_count: Option<i64>,

/// 
    #[serde(rename = "RemoteDirtyByteCount")]
    pub remote_dirty_byte_count: Option<i64>,
}

impl Win32_OfflineFilesDirtyInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            local_dirty_byte_count: None,
            remote_dirty_byte_count: None,
        }
    }


    /// Sets the value of LocalDirtyByteCount
    pub fn set_local_dirty_byte_count(&mut self, value: i64) {
        self.local_dirty_byte_count = Some(value);
    }

    /// Gets the value of LocalDirtyByteCount
    pub fn get_local_dirty_byte_count(&self) -> Option<&i64> {
        self.local_dirty_byte_count.as_ref()
    }

    /// Sets the value of RemoteDirtyByteCount
    pub fn set_remote_dirty_byte_count(&mut self, value: i64) {
        self.remote_dirty_byte_count = Some(value);
    }

    /// Gets the value of RemoteDirtyByteCount
    pub fn get_remote_dirty_byte_count(&self) -> Option<&i64> {
        self.remote_dirty_byte_count.as_ref()
    }
}

