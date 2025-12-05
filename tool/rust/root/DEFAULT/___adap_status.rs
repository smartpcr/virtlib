// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.DEFAULT
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __AdapStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __AdapStatus {
    #[serde(flatten)]
    pub base: __SystemClass,

/// 
    #[serde(rename = "LastStartTime")]
    pub last_start_time: Option<String>,

/// 
    #[serde(rename = "LastStopTime")]
    pub last_stop_time: Option<String>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u32>,
}

impl __AdapStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __SystemClass::new(),
            last_start_time: None,
            last_stop_time: None,
            status: None,
        }
    }


    /// Sets the value of LastStartTime
    pub fn set_last_start_time(&mut self, value: String) {
        self.last_start_time = Some(value);
    }

    /// Gets the value of LastStartTime
    pub fn get_last_start_time(&self) -> Option<&String> {
        self.last_start_time.as_ref()
    }

    /// Sets the value of LastStopTime
    pub fn set_last_stop_time(&mut self, value: String) {
        self.last_stop_time = Some(value);
    }

    /// Gets the value of LastStopTime
    pub fn get_last_stop_time(&self) -> Option<&String> {
        self.last_stop_time.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u32> {
        self.status.as_ref()
    }
}

