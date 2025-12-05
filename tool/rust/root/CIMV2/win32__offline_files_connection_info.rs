// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_OfflineFilesConnectionInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_OfflineFilesConnectionInfo {

/// 
    #[serde(rename = "ConnectState")]
    pub connect_state: Option<u32>,

/// 
    #[serde(rename = "OfflineReason")]
    pub offline_reason: Option<u32>,
}

impl Win32_OfflineFilesConnectionInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            connect_state: None,
            offline_reason: None,
        }
    }


    /// Sets the value of ConnectState
    pub fn set_connect_state(&mut self, value: u32) {
        self.connect_state = Some(value);
    }

    /// Gets the value of ConnectState
    pub fn get_connect_state(&self) -> Option<&u32> {
        self.connect_state.as_ref()
    }

    /// Sets the value of OfflineReason
    pub fn set_offline_reason(&mut self, value: u32) {
        self.offline_reason = Some(value);
    }

    /// Gets the value of OfflineReason
    pub fn get_offline_reason(&self) -> Option<&u32> {
        self.offline_reason.as_ref()
    }
}

