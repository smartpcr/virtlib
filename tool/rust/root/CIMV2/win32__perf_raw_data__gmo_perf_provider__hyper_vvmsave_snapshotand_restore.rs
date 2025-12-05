// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_GmoPerfProvider_HyperVVMSaveSnapshotandRestore struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_GmoPerfProvider_HyperVVMSaveSnapshotandRestore {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "OperationTime")]
    pub operation_time: Option<u32>,

/// 
    #[serde(rename = "RequestsActive")]
    pub requests_active: Option<u32>,

/// 
    #[serde(rename = "RequestsDispatched")]
    pub requests_dispatched: Option<u32>,

/// 
    #[serde(rename = "RequestsHighPriority")]
    pub requests_high_priority: Option<u32>,

/// 
    #[serde(rename = "RequestsProcessed")]
    pub requests_processed: Option<u32>,

/// 
    #[serde(rename = "ThreadsSpawned")]
    pub threads_spawned: Option<u32>,
}

impl Win32_PerfRawData_GmoPerfProvider_HyperVVMSaveSnapshotandRestore {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            operation_time: None,
            requests_active: None,
            requests_dispatched: None,
            requests_high_priority: None,
            requests_processed: None,
            threads_spawned: None,
        }
    }


    /// Sets the value of OperationTime
    pub fn set_operation_time(&mut self, value: u32) {
        self.operation_time = Some(value);
    }

    /// Gets the value of OperationTime
    pub fn get_operation_time(&self) -> Option<&u32> {
        self.operation_time.as_ref()
    }

    /// Sets the value of RequestsActive
    pub fn set_requests_active(&mut self, value: u32) {
        self.requests_active = Some(value);
    }

    /// Gets the value of RequestsActive
    pub fn get_requests_active(&self) -> Option<&u32> {
        self.requests_active.as_ref()
    }

    /// Sets the value of RequestsDispatched
    pub fn set_requests_dispatched(&mut self, value: u32) {
        self.requests_dispatched = Some(value);
    }

    /// Gets the value of RequestsDispatched
    pub fn get_requests_dispatched(&self) -> Option<&u32> {
        self.requests_dispatched.as_ref()
    }

    /// Sets the value of RequestsHighPriority
    pub fn set_requests_high_priority(&mut self, value: u32) {
        self.requests_high_priority = Some(value);
    }

    /// Gets the value of RequestsHighPriority
    pub fn get_requests_high_priority(&self) -> Option<&u32> {
        self.requests_high_priority.as_ref()
    }

    /// Sets the value of RequestsProcessed
    pub fn set_requests_processed(&mut self, value: u32) {
        self.requests_processed = Some(value);
    }

    /// Gets the value of RequestsProcessed
    pub fn get_requests_processed(&self) -> Option<&u32> {
        self.requests_processed.as_ref()
    }

    /// Sets the value of ThreadsSpawned
    pub fn set_threads_spawned(&mut self, value: u32) {
        self.threads_spawned = Some(value);
    }

    /// Gets the value of ThreadsSpawned
    pub fn get_threads_spawned(&self) -> Option<&u32> {
        self.threads_spawned.as_ref()
    }
}

