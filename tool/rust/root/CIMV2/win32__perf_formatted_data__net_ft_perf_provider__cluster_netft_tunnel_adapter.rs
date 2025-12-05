// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_NetFtPerfProvider_ClusterNetftTunnelAdapter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_NetFtPerfProvider_ClusterNetftTunnelAdapter {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "TotalReceives")]
    pub total_receives: Option<u64>,

/// 
    #[serde(rename = "TotalReceivesAccepted")]
    pub total_receives_accepted: Option<u64>,

/// 
    #[serde(rename = "TotalReceivesDropped")]
    pub total_receives_dropped: Option<u64>,

/// 
    #[serde(rename = "TotalSendRequests")]
    pub total_send_requests: Option<u64>,

/// 
    #[serde(rename = "TotalSendRequestsAccepted")]
    pub total_send_requests_accepted: Option<u64>,

/// 
    #[serde(rename = "TotalSendRequestsDropped")]
    pub total_send_requests_dropped: Option<u64>,
}

impl Win32_PerfFormattedData_NetFtPerfProvider_ClusterNetftTunnelAdapter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            total_receives: None,
            total_receives_accepted: None,
            total_receives_dropped: None,
            total_send_requests: None,
            total_send_requests_accepted: None,
            total_send_requests_dropped: None,
        }
    }


    /// Sets the value of TotalReceives
    pub fn set_total_receives(&mut self, value: u64) {
        self.total_receives = Some(value);
    }

    /// Gets the value of TotalReceives
    pub fn get_total_receives(&self) -> Option<&u64> {
        self.total_receives.as_ref()
    }

    /// Sets the value of TotalReceivesAccepted
    pub fn set_total_receives_accepted(&mut self, value: u64) {
        self.total_receives_accepted = Some(value);
    }

    /// Gets the value of TotalReceivesAccepted
    pub fn get_total_receives_accepted(&self) -> Option<&u64> {
        self.total_receives_accepted.as_ref()
    }

    /// Sets the value of TotalReceivesDropped
    pub fn set_total_receives_dropped(&mut self, value: u64) {
        self.total_receives_dropped = Some(value);
    }

    /// Gets the value of TotalReceivesDropped
    pub fn get_total_receives_dropped(&self) -> Option<&u64> {
        self.total_receives_dropped.as_ref()
    }

    /// Sets the value of TotalSendRequests
    pub fn set_total_send_requests(&mut self, value: u64) {
        self.total_send_requests = Some(value);
    }

    /// Gets the value of TotalSendRequests
    pub fn get_total_send_requests(&self) -> Option<&u64> {
        self.total_send_requests.as_ref()
    }

    /// Sets the value of TotalSendRequestsAccepted
    pub fn set_total_send_requests_accepted(&mut self, value: u64) {
        self.total_send_requests_accepted = Some(value);
    }

    /// Gets the value of TotalSendRequestsAccepted
    pub fn get_total_send_requests_accepted(&self) -> Option<&u64> {
        self.total_send_requests_accepted.as_ref()
    }

    /// Sets the value of TotalSendRequestsDropped
    pub fn set_total_send_requests_dropped(&mut self, value: u64) {
        self.total_send_requests_dropped = Some(value);
    }

    /// Gets the value of TotalSendRequestsDropped
    pub fn get_total_send_requests_dropped(&self) -> Option<&u64> {
        self.total_send_requests_dropped.as_ref()
    }
}

