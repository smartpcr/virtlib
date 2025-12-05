// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetServiceCrashNoAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetServiceCrashNoAction {
    #[serde(flatten)]
    pub base: MSFT_SCMEventLogEvent,

/// 
    #[serde(rename = "Service")]
    pub service: Option<String>,

/// 
    #[serde(rename = "TimesFailed")]
    pub times_failed: Option<u32>,
}

impl MSFT_NetServiceCrashNoAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_SCMEventLogEvent::new(),
            service: None,
            times_failed: None,
        }
    }


    /// Sets the value of Service
    pub fn set_service(&mut self, value: String) {
        self.service = Some(value);
    }

    /// Gets the value of Service
    pub fn get_service(&self) -> Option<&String> {
        self.service.as_ref()
    }

    /// Sets the value of TimesFailed
    pub fn set_times_failed(&mut self, value: u32) {
        self.times_failed = Some(value);
    }

    /// Gets the value of TimesFailed
    pub fn get_times_failed(&self) -> Option<&u32> {
        self.times_failed.as_ref()
    }
}

