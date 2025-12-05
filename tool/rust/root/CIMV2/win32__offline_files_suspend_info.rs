// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_OfflineFilesSuspendInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_OfflineFilesSuspendInfo {

/// 
    #[serde(rename = "Suspended")]
    pub suspended: Option<bool>,

/// 
    #[serde(rename = "SuspendedRoot")]
    pub suspended_root: Option<bool>,
}

impl Win32_OfflineFilesSuspendInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            suspended: None,
            suspended_root: None,
        }
    }


    /// Sets the value of Suspended
    pub fn set_suspended(&mut self, value: bool) {
        self.suspended = Some(value);
    }

    /// Gets the value of Suspended
    pub fn get_suspended(&self) -> Option<&bool> {
        self.suspended.as_ref()
    }

    /// Sets the value of SuspendedRoot
    pub fn set_suspended_root(&mut self, value: bool) {
        self.suspended_root = Some(value);
    }

    /// Gets the value of SuspendedRoot
    pub fn get_suspended_root(&self) -> Option<&bool> {
        self.suspended_root.as_ref()
    }
}

