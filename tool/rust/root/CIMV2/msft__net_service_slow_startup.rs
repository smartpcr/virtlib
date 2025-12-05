// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetServiceSlowStartup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetServiceSlowStartup {
    #[serde(flatten)]
    pub base: MSFT_SCMEventLogEvent,

/// 
    #[serde(rename = "Service")]
    pub service: Option<String>,

/// 
    #[serde(rename = "StartupTime")]
    pub startup_time: Option<u32>,
}

impl MSFT_NetServiceSlowStartup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_SCMEventLogEvent::new(),
            service: None,
            startup_time: None,
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

    /// Sets the value of StartupTime
    pub fn set_startup_time(&mut self, value: u32) {
        self.startup_time = Some(value);
    }

    /// Gets the value of StartupTime
    pub fn get_startup_time(&self) -> Option<&u32> {
        self.startup_time.as_ref()
    }
}

