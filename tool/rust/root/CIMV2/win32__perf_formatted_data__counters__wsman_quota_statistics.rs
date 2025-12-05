// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_WSManQuotaStatistics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_WSManQuotaStatistics {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "ActiveOperations")]
    pub active_operations: Option<u32>,

/// 
    #[serde(rename = "ActiveShells")]
    pub active_shells: Option<u32>,

/// 
    #[serde(rename = "ActiveUsers")]
    pub active_users: Option<u32>,

/// 
    #[serde(rename = "ProcessID")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "SystemQuotaViolationsPerSecond")]
    pub system_quota_violations_per_second: Option<u32>,

/// 
    #[serde(rename = "TotalRequestsPerSecond")]
    pub total_requests_per_second: Option<u32>,

/// 
    #[serde(rename = "UserQuotaViolationsPerSecond")]
    pub user_quota_violations_per_second: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_WSManQuotaStatistics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            active_operations: None,
            active_shells: None,
            active_users: None,
            process_id: None,
            system_quota_violations_per_second: None,
            total_requests_per_second: None,
            user_quota_violations_per_second: None,
        }
    }


    /// Sets the value of ActiveOperations
    pub fn set_active_operations(&mut self, value: u32) {
        self.active_operations = Some(value);
    }

    /// Gets the value of ActiveOperations
    pub fn get_active_operations(&self) -> Option<&u32> {
        self.active_operations.as_ref()
    }

    /// Sets the value of ActiveShells
    pub fn set_active_shells(&mut self, value: u32) {
        self.active_shells = Some(value);
    }

    /// Gets the value of ActiveShells
    pub fn get_active_shells(&self) -> Option<&u32> {
        self.active_shells.as_ref()
    }

    /// Sets the value of ActiveUsers
    pub fn set_active_users(&mut self, value: u32) {
        self.active_users = Some(value);
    }

    /// Gets the value of ActiveUsers
    pub fn get_active_users(&self) -> Option<&u32> {
        self.active_users.as_ref()
    }

    /// Sets the value of ProcessID
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessID
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of SystemQuotaViolationsPerSecond
    pub fn set_system_quota_violations_per_second(&mut self, value: u32) {
        self.system_quota_violations_per_second = Some(value);
    }

    /// Gets the value of SystemQuotaViolationsPerSecond
    pub fn get_system_quota_violations_per_second(&self) -> Option<&u32> {
        self.system_quota_violations_per_second.as_ref()
    }

    /// Sets the value of TotalRequestsPerSecond
    pub fn set_total_requests_per_second(&mut self, value: u32) {
        self.total_requests_per_second = Some(value);
    }

    /// Gets the value of TotalRequestsPerSecond
    pub fn get_total_requests_per_second(&self) -> Option<&u32> {
        self.total_requests_per_second.as_ref()
    }

    /// Sets the value of UserQuotaViolationsPerSecond
    pub fn set_user_quota_violations_per_second(&mut self, value: u32) {
        self.user_quota_violations_per_second = Some(value);
    }

    /// Gets the value of UserQuotaViolationsPerSecond
    pub fn get_user_quota_violations_per_second(&self) -> Option<&u32> {
        self.user_quota_violations_per_second.as_ref()
    }
}

