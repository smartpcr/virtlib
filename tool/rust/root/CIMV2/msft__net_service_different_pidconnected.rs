// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetServiceDifferentPIDConnected struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetServiceDifferentPIDConnected {
    #[serde(flatten)]
    pub base: MSFT_SCMEventLogEvent,

/// 
    #[serde(rename = "ActualPID")]
    pub actual_pid: Option<u32>,

/// 
    #[serde(rename = "ExpectedPID")]
    pub expected_pid: Option<u32>,

/// 
    #[serde(rename = "Service")]
    pub service: Option<String>,
}

impl MSFT_NetServiceDifferentPIDConnected {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_SCMEventLogEvent::new(),
            actual_pid: None,
            expected_pid: None,
            service: None,
        }
    }


    /// Sets the value of ActualPID
    pub fn set_actual_pid(&mut self, value: u32) {
        self.actual_pid = Some(value);
    }

    /// Gets the value of ActualPID
    pub fn get_actual_pid(&self) -> Option<&u32> {
        self.actual_pid.as_ref()
    }

    /// Sets the value of ExpectedPID
    pub fn set_expected_pid(&mut self, value: u32) {
        self.expected_pid = Some(value);
    }

    /// Gets the value of ExpectedPID
    pub fn get_expected_pid(&self) -> Option<&u32> {
        self.expected_pid.as_ref()
    }

    /// Sets the value of Service
    pub fn set_service(&mut self, value: String) {
        self.service = Some(value);
    }

    /// Gets the value of Service
    pub fn get_service(&self) -> Option<&String> {
        self.service.as_ref()
    }
}

