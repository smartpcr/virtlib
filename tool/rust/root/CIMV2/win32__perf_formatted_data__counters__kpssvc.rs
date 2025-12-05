// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_KPSSVC struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_KPSSVC {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "FailedRequests")]
    pub failed_requests: Option<u32>,

/// 
    #[serde(rename = "IncomingArmoredRequests")]
    pub incoming_armored_requests: Option<u32>,

/// 
    #[serde(rename = "IncomingPasswordChangeRequests")]
    pub incoming_password_change_requests: Option<u32>,

/// 
    #[serde(rename = "IncomingRequests")]
    pub incoming_requests: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_KPSSVC {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            failed_requests: None,
            incoming_armored_requests: None,
            incoming_password_change_requests: None,
            incoming_requests: None,
        }
    }


    /// Sets the value of FailedRequests
    pub fn set_failed_requests(&mut self, value: u32) {
        self.failed_requests = Some(value);
    }

    /// Gets the value of FailedRequests
    pub fn get_failed_requests(&self) -> Option<&u32> {
        self.failed_requests.as_ref()
    }

    /// Sets the value of IncomingArmoredRequests
    pub fn set_incoming_armored_requests(&mut self, value: u32) {
        self.incoming_armored_requests = Some(value);
    }

    /// Gets the value of IncomingArmoredRequests
    pub fn get_incoming_armored_requests(&self) -> Option<&u32> {
        self.incoming_armored_requests.as_ref()
    }

    /// Sets the value of IncomingPasswordChangeRequests
    pub fn set_incoming_password_change_requests(&mut self, value: u32) {
        self.incoming_password_change_requests = Some(value);
    }

    /// Gets the value of IncomingPasswordChangeRequests
    pub fn get_incoming_password_change_requests(&self) -> Option<&u32> {
        self.incoming_password_change_requests.as_ref()
    }

    /// Sets the value of IncomingRequests
    pub fn set_incoming_requests(&mut self, value: u32) {
        self.incoming_requests = Some(value);
    }

    /// Gets the value of IncomingRequests
    pub fn get_incoming_requests(&self) -> Option<&u32> {
        self.incoming_requests.as_ref()
    }
}

