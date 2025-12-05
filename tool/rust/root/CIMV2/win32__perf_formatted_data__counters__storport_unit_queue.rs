// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_StorportUnitQueue struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_StorportUnitQueue {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "OutstandingRequests")]
    pub outstanding_requests: Option<u32>,

/// 
    #[serde(rename = "QueuedRequests")]
    pub queued_requests: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_StorportUnitQueue {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            outstanding_requests: None,
            queued_requests: None,
        }
    }


    /// Sets the value of OutstandingRequests
    pub fn set_outstanding_requests(&mut self, value: u32) {
        self.outstanding_requests = Some(value);
    }

    /// Gets the value of OutstandingRequests
    pub fn get_outstanding_requests(&self) -> Option<&u32> {
        self.outstanding_requests.as_ref()
    }

    /// Sets the value of QueuedRequests
    pub fn set_queued_requests(&mut self, value: u32) {
        self.queued_requests = Some(value);
    }

    /// Gets the value of QueuedRequests
    pub fn get_queued_requests(&self) -> Option<&u32> {
        self.queued_requests.as_ref()
    }
}

