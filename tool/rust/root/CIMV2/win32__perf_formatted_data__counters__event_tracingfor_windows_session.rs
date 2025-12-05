// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_EventTracingforWindowsSession struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_EventTracingforWindowsSession {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "BufferMemoryUsageNonPagedPool")]
    pub buffer_memory_usage_non_paged_pool: Option<u32>,

/// 
    #[serde(rename = "BufferMemoryUsagePagedPool")]
    pub buffer_memory_usage_paged_pool: Option<u32>,

/// 
    #[serde(rename = "EventsLoggedpersec")]
    pub events_loggedpersec: Option<u64>,

/// 
    #[serde(rename = "EventsLost")]
    pub events_lost: Option<u32>,

/// 
    #[serde(rename = "NumberofRealTimeConsumers")]
    pub numberof_real_time_consumers: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_EventTracingforWindowsSession {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            buffer_memory_usage_non_paged_pool: None,
            buffer_memory_usage_paged_pool: None,
            events_loggedpersec: None,
            events_lost: None,
            numberof_real_time_consumers: None,
        }
    }


    /// Sets the value of BufferMemoryUsageNonPagedPool
    pub fn set_buffer_memory_usage_non_paged_pool(&mut self, value: u32) {
        self.buffer_memory_usage_non_paged_pool = Some(value);
    }

    /// Gets the value of BufferMemoryUsageNonPagedPool
    pub fn get_buffer_memory_usage_non_paged_pool(&self) -> Option<&u32> {
        self.buffer_memory_usage_non_paged_pool.as_ref()
    }

    /// Sets the value of BufferMemoryUsagePagedPool
    pub fn set_buffer_memory_usage_paged_pool(&mut self, value: u32) {
        self.buffer_memory_usage_paged_pool = Some(value);
    }

    /// Gets the value of BufferMemoryUsagePagedPool
    pub fn get_buffer_memory_usage_paged_pool(&self) -> Option<&u32> {
        self.buffer_memory_usage_paged_pool.as_ref()
    }

    /// Sets the value of EventsLoggedpersec
    pub fn set_events_loggedpersec(&mut self, value: u64) {
        self.events_loggedpersec = Some(value);
    }

    /// Gets the value of EventsLoggedpersec
    pub fn get_events_loggedpersec(&self) -> Option<&u64> {
        self.events_loggedpersec.as_ref()
    }

    /// Sets the value of EventsLost
    pub fn set_events_lost(&mut self, value: u32) {
        self.events_lost = Some(value);
    }

    /// Gets the value of EventsLost
    pub fn get_events_lost(&self) -> Option<&u32> {
        self.events_lost.as_ref()
    }

    /// Sets the value of NumberofRealTimeConsumers
    pub fn set_numberof_real_time_consumers(&mut self, value: u32) {
        self.numberof_real_time_consumers = Some(value);
    }

    /// Gets the value of NumberofRealTimeConsumers
    pub fn get_numberof_real_time_consumers(&self) -> Option<&u32> {
        self.numberof_real_time_consumers.as_ref()
    }
}

