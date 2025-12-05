// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_Netlogon struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_Netlogon {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "AverageSemaphoreHoldTime")]
    pub average_semaphore_hold_time: Option<u32>,

/// 
    #[serde(rename = "LastAuthenticationTime")]
    pub last_authentication_time: Option<u32>,

/// 
    #[serde(rename = "SemaphoreAcquires")]
    pub semaphore_acquires: Option<u64>,

/// 
    #[serde(rename = "SemaphoreHolders")]
    pub semaphore_holders: Option<u32>,

/// 
    #[serde(rename = "SemaphoreTimeouts")]
    pub semaphore_timeouts: Option<u64>,

/// 
    #[serde(rename = "SemaphoreWaiters")]
    pub semaphore_waiters: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_Netlogon {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            average_semaphore_hold_time: None,
            last_authentication_time: None,
            semaphore_acquires: None,
            semaphore_holders: None,
            semaphore_timeouts: None,
            semaphore_waiters: None,
        }
    }


    /// Sets the value of AverageSemaphoreHoldTime
    pub fn set_average_semaphore_hold_time(&mut self, value: u32) {
        self.average_semaphore_hold_time = Some(value);
    }

    /// Gets the value of AverageSemaphoreHoldTime
    pub fn get_average_semaphore_hold_time(&self) -> Option<&u32> {
        self.average_semaphore_hold_time.as_ref()
    }

    /// Sets the value of LastAuthenticationTime
    pub fn set_last_authentication_time(&mut self, value: u32) {
        self.last_authentication_time = Some(value);
    }

    /// Gets the value of LastAuthenticationTime
    pub fn get_last_authentication_time(&self) -> Option<&u32> {
        self.last_authentication_time.as_ref()
    }

    /// Sets the value of SemaphoreAcquires
    pub fn set_semaphore_acquires(&mut self, value: u64) {
        self.semaphore_acquires = Some(value);
    }

    /// Gets the value of SemaphoreAcquires
    pub fn get_semaphore_acquires(&self) -> Option<&u64> {
        self.semaphore_acquires.as_ref()
    }

    /// Sets the value of SemaphoreHolders
    pub fn set_semaphore_holders(&mut self, value: u32) {
        self.semaphore_holders = Some(value);
    }

    /// Gets the value of SemaphoreHolders
    pub fn get_semaphore_holders(&self) -> Option<&u32> {
        self.semaphore_holders.as_ref()
    }

    /// Sets the value of SemaphoreTimeouts
    pub fn set_semaphore_timeouts(&mut self, value: u64) {
        self.semaphore_timeouts = Some(value);
    }

    /// Gets the value of SemaphoreTimeouts
    pub fn get_semaphore_timeouts(&self) -> Option<&u64> {
        self.semaphore_timeouts.as_ref()
    }

    /// Sets the value of SemaphoreWaiters
    pub fn set_semaphore_waiters(&mut self, value: u32) {
        self.semaphore_waiters = Some(value);
    }

    /// Gets the value of SemaphoreWaiters
    pub fn get_semaphore_waiters(&self) -> Option<&u32> {
        self.semaphore_waiters.as_ref()
    }
}

