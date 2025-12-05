// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_WMIProviderHost_WMIPrvSEHealthStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_WMIProviderHost_WMIPrvSEHealthStatus {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "CPUPercentConsumption")]
    pub cpupercent_consumption: Option<u64>,

/// 
    #[serde(rename = "HandleCount")]
    pub handle_count: Option<u32>,

/// 
    #[serde(rename = "PrivatePageMemory")]
    pub private_page_memory: Option<u64>,

/// 
    #[serde(rename = "ProcessID")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "ThreadCount")]
    pub thread_count: Option<u32>,

/// 
    #[serde(rename = "TimestampOfHealthStatus")]
    pub timestamp_of_health_status: Option<u64>,
}

impl Win32_PerfFormattedData_WMIProviderHost_WMIPrvSEHealthStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            cpupercent_consumption: None,
            handle_count: None,
            private_page_memory: None,
            process_id: None,
            thread_count: None,
            timestamp_of_health_status: None,
        }
    }


    /// Sets the value of CPUPercentConsumption
    pub fn set_cpupercent_consumption(&mut self, value: u64) {
        self.cpupercent_consumption = Some(value);
    }

    /// Gets the value of CPUPercentConsumption
    pub fn get_cpupercent_consumption(&self) -> Option<&u64> {
        self.cpupercent_consumption.as_ref()
    }

    /// Sets the value of HandleCount
    pub fn set_handle_count(&mut self, value: u32) {
        self.handle_count = Some(value);
    }

    /// Gets the value of HandleCount
    pub fn get_handle_count(&self) -> Option<&u32> {
        self.handle_count.as_ref()
    }

    /// Sets the value of PrivatePageMemory
    pub fn set_private_page_memory(&mut self, value: u64) {
        self.private_page_memory = Some(value);
    }

    /// Gets the value of PrivatePageMemory
    pub fn get_private_page_memory(&self) -> Option<&u64> {
        self.private_page_memory.as_ref()
    }

    /// Sets the value of ProcessID
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessID
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of ThreadCount
    pub fn set_thread_count(&mut self, value: u32) {
        self.thread_count = Some(value);
    }

    /// Gets the value of ThreadCount
    pub fn get_thread_count(&self) -> Option<&u32> {
        self.thread_count.as_ref()
    }

    /// Sets the value of TimestampOfHealthStatus
    pub fn set_timestamp_of_health_status(&mut self, value: u64) {
        self.timestamp_of_health_status = Some(value);
    }

    /// Gets the value of TimestampOfHealthStatus
    pub fn get_timestamp_of_health_status(&self) -> Option<&u64> {
        self.timestamp_of_health_status.as_ref()
    }
}

