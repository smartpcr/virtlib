// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_EventTracingforWindows struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_EventTracingforWindows {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "TotalMemoryUsageNonPagedPool")]
    pub total_memory_usage_non_paged_pool: Option<u32>,

/// 
    #[serde(rename = "TotalMemoryUsagePagedPool")]
    pub total_memory_usage_paged_pool: Option<u32>,

/// 
    #[serde(rename = "TotalNumberofActiveSessions")]
    pub total_numberof_active_sessions: Option<u32>,

/// 
    #[serde(rename = "TotalNumberofDistinctDisabledProviders")]
    pub total_numberof_distinct_disabled_providers: Option<u32>,

/// 
    #[serde(rename = "TotalNumberofDistinctEnabledProviders")]
    pub total_numberof_distinct_enabled_providers: Option<u32>,

/// 
    #[serde(rename = "TotalNumberofDistinctPreEnabledProviders")]
    pub total_numberof_distinct_pre_enabled_providers: Option<u32>,
}

impl Win32_PerfRawData_Counters_EventTracingforWindows {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            total_memory_usage_non_paged_pool: None,
            total_memory_usage_paged_pool: None,
            total_numberof_active_sessions: None,
            total_numberof_distinct_disabled_providers: None,
            total_numberof_distinct_enabled_providers: None,
            total_numberof_distinct_pre_enabled_providers: None,
        }
    }


    /// Sets the value of TotalMemoryUsageNonPagedPool
    pub fn set_total_memory_usage_non_paged_pool(&mut self, value: u32) {
        self.total_memory_usage_non_paged_pool = Some(value);
    }

    /// Gets the value of TotalMemoryUsageNonPagedPool
    pub fn get_total_memory_usage_non_paged_pool(&self) -> Option<&u32> {
        self.total_memory_usage_non_paged_pool.as_ref()
    }

    /// Sets the value of TotalMemoryUsagePagedPool
    pub fn set_total_memory_usage_paged_pool(&mut self, value: u32) {
        self.total_memory_usage_paged_pool = Some(value);
    }

    /// Gets the value of TotalMemoryUsagePagedPool
    pub fn get_total_memory_usage_paged_pool(&self) -> Option<&u32> {
        self.total_memory_usage_paged_pool.as_ref()
    }

    /// Sets the value of TotalNumberofActiveSessions
    pub fn set_total_numberof_active_sessions(&mut self, value: u32) {
        self.total_numberof_active_sessions = Some(value);
    }

    /// Gets the value of TotalNumberofActiveSessions
    pub fn get_total_numberof_active_sessions(&self) -> Option<&u32> {
        self.total_numberof_active_sessions.as_ref()
    }

    /// Sets the value of TotalNumberofDistinctDisabledProviders
    pub fn set_total_numberof_distinct_disabled_providers(&mut self, value: u32) {
        self.total_numberof_distinct_disabled_providers = Some(value);
    }

    /// Gets the value of TotalNumberofDistinctDisabledProviders
    pub fn get_total_numberof_distinct_disabled_providers(&self) -> Option<&u32> {
        self.total_numberof_distinct_disabled_providers.as_ref()
    }

    /// Sets the value of TotalNumberofDistinctEnabledProviders
    pub fn set_total_numberof_distinct_enabled_providers(&mut self, value: u32) {
        self.total_numberof_distinct_enabled_providers = Some(value);
    }

    /// Gets the value of TotalNumberofDistinctEnabledProviders
    pub fn get_total_numberof_distinct_enabled_providers(&self) -> Option<&u32> {
        self.total_numberof_distinct_enabled_providers.as_ref()
    }

    /// Sets the value of TotalNumberofDistinctPreEnabledProviders
    pub fn set_total_numberof_distinct_pre_enabled_providers(&mut self, value: u32) {
        self.total_numberof_distinct_pre_enabled_providers = Some(value);
    }

    /// Gets the value of TotalNumberofDistinctPreEnabledProviders
    pub fn get_total_numberof_distinct_pre_enabled_providers(&self) -> Option<&u32> {
        self.total_numberof_distinct_pre_enabled_providers.as_ref()
    }
}

