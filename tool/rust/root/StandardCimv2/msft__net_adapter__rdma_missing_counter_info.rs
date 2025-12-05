// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapter_RdmaMissingCounterInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapter_RdmaMissingCounterInfo {

/// 
    #[serde(rename = "AcceptPerformanceCounterMissing")]
    pub accept_performance_counter_missing: Option<bool>,

/// 
    #[serde(rename = "ActiveConnectionPerformanceCounterMissing")]
    pub active_connection_performance_counter_missing: Option<bool>,

/// 
    #[serde(rename = "CompletionQueueErrorPerformanceCounterMissing")]
    pub completion_queue_error_performance_counter_missing: Option<bool>,

/// 
    #[serde(rename = "ConnectFailurePerformanceCounterMissing")]
    pub connect_failure_performance_counter_missing: Option<bool>,

/// 
    #[serde(rename = "ConnectionErrorPerformanceCounterMissing")]
    pub connection_error_performance_counter_missing: Option<bool>,

/// 
    #[serde(rename = "ConnectPerformanceCounterMissing")]
    pub connect_performance_counter_missing: Option<bool>,

/// 
    #[serde(rename = "RDMAInFramesPerformanceCounterMissing")]
    pub rdmain_frames_performance_counter_missing: Option<bool>,

/// 
    #[serde(rename = "RDMAInOctetsPerformanceCounterMissing")]
    pub rdmain_octets_performance_counter_missing: Option<bool>,

/// 
    #[serde(rename = "RDMAOutFramesPerformanceCounterMissing")]
    pub rdmaout_frames_performance_counter_missing: Option<bool>,

/// 
    #[serde(rename = "RDMAOutOctetsPerformanceCounterMissing")]
    pub rdmaout_octets_performance_counter_missing: Option<bool>,
}

impl MSFT_NetAdapter_RdmaMissingCounterInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            accept_performance_counter_missing: None,
            active_connection_performance_counter_missing: None,
            completion_queue_error_performance_counter_missing: None,
            connect_failure_performance_counter_missing: None,
            connection_error_performance_counter_missing: None,
            connect_performance_counter_missing: None,
            rdmain_frames_performance_counter_missing: None,
            rdmain_octets_performance_counter_missing: None,
            rdmaout_frames_performance_counter_missing: None,
            rdmaout_octets_performance_counter_missing: None,
        }
    }


    /// Sets the value of AcceptPerformanceCounterMissing
    pub fn set_accept_performance_counter_missing(&mut self, value: bool) {
        self.accept_performance_counter_missing = Some(value);
    }

    /// Gets the value of AcceptPerformanceCounterMissing
    pub fn get_accept_performance_counter_missing(&self) -> Option<&bool> {
        self.accept_performance_counter_missing.as_ref()
    }

    /// Sets the value of ActiveConnectionPerformanceCounterMissing
    pub fn set_active_connection_performance_counter_missing(&mut self, value: bool) {
        self.active_connection_performance_counter_missing = Some(value);
    }

    /// Gets the value of ActiveConnectionPerformanceCounterMissing
    pub fn get_active_connection_performance_counter_missing(&self) -> Option<&bool> {
        self.active_connection_performance_counter_missing.as_ref()
    }

    /// Sets the value of CompletionQueueErrorPerformanceCounterMissing
    pub fn set_completion_queue_error_performance_counter_missing(&mut self, value: bool) {
        self.completion_queue_error_performance_counter_missing = Some(value);
    }

    /// Gets the value of CompletionQueueErrorPerformanceCounterMissing
    pub fn get_completion_queue_error_performance_counter_missing(&self) -> Option<&bool> {
        self.completion_queue_error_performance_counter_missing.as_ref()
    }

    /// Sets the value of ConnectFailurePerformanceCounterMissing
    pub fn set_connect_failure_performance_counter_missing(&mut self, value: bool) {
        self.connect_failure_performance_counter_missing = Some(value);
    }

    /// Gets the value of ConnectFailurePerformanceCounterMissing
    pub fn get_connect_failure_performance_counter_missing(&self) -> Option<&bool> {
        self.connect_failure_performance_counter_missing.as_ref()
    }

    /// Sets the value of ConnectionErrorPerformanceCounterMissing
    pub fn set_connection_error_performance_counter_missing(&mut self, value: bool) {
        self.connection_error_performance_counter_missing = Some(value);
    }

    /// Gets the value of ConnectionErrorPerformanceCounterMissing
    pub fn get_connection_error_performance_counter_missing(&self) -> Option<&bool> {
        self.connection_error_performance_counter_missing.as_ref()
    }

    /// Sets the value of ConnectPerformanceCounterMissing
    pub fn set_connect_performance_counter_missing(&mut self, value: bool) {
        self.connect_performance_counter_missing = Some(value);
    }

    /// Gets the value of ConnectPerformanceCounterMissing
    pub fn get_connect_performance_counter_missing(&self) -> Option<&bool> {
        self.connect_performance_counter_missing.as_ref()
    }

    /// Sets the value of RDMAInFramesPerformanceCounterMissing
    pub fn set_rdmain_frames_performance_counter_missing(&mut self, value: bool) {
        self.rdmain_frames_performance_counter_missing = Some(value);
    }

    /// Gets the value of RDMAInFramesPerformanceCounterMissing
    pub fn get_rdmain_frames_performance_counter_missing(&self) -> Option<&bool> {
        self.rdmain_frames_performance_counter_missing.as_ref()
    }

    /// Sets the value of RDMAInOctetsPerformanceCounterMissing
    pub fn set_rdmain_octets_performance_counter_missing(&mut self, value: bool) {
        self.rdmain_octets_performance_counter_missing = Some(value);
    }

    /// Gets the value of RDMAInOctetsPerformanceCounterMissing
    pub fn get_rdmain_octets_performance_counter_missing(&self) -> Option<&bool> {
        self.rdmain_octets_performance_counter_missing.as_ref()
    }

    /// Sets the value of RDMAOutFramesPerformanceCounterMissing
    pub fn set_rdmaout_frames_performance_counter_missing(&mut self, value: bool) {
        self.rdmaout_frames_performance_counter_missing = Some(value);
    }

    /// Gets the value of RDMAOutFramesPerformanceCounterMissing
    pub fn get_rdmaout_frames_performance_counter_missing(&self) -> Option<&bool> {
        self.rdmaout_frames_performance_counter_missing.as_ref()
    }

    /// Sets the value of RDMAOutOctetsPerformanceCounterMissing
    pub fn set_rdmaout_octets_performance_counter_missing(&mut self, value: bool) {
        self.rdmaout_octets_performance_counter_missing = Some(value);
    }

    /// Gets the value of RDMAOutOctetsPerformanceCounterMissing
    pub fn get_rdmaout_octets_performance_counter_missing(&self) -> Option<&bool> {
        self.rdmaout_octets_performance_counter_missing.as_ref()
    }
}

