// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __NotifyStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __NotifyStatus {

/// 
    #[serde(rename = "StatusCode")]
    pub status_code: Option<u32>,
}

impl __NotifyStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            status_code: None,
        }
    }


    /// Sets the value of StatusCode
    pub fn set_status_code(&mut self, value: u32) {
        self.status_code = Some(value);
    }

    /// Gets the value of StatusCode
    pub fn get_status_code(&self) -> Option<&u32> {
        self.status_code.as_ref()
    }
}

