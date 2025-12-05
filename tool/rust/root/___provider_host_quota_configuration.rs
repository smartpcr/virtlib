// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __ProviderHostQuotaConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __ProviderHostQuotaConfiguration {
    #[serde(flatten)]
    pub base: __SystemClass,

/// 
    #[serde(rename = "HandlesPerHost")]
    pub handles_per_host: Option<u32>,

/// 
    #[serde(rename = "MemoryAllHosts")]
    pub memory_all_hosts: Option<u64>,

/// 
    #[serde(rename = "MemoryPerHost")]
    pub memory_per_host: Option<u64>,

/// 
    #[serde(rename = "ProcessLimitAllHosts")]
    pub process_limit_all_hosts: Option<u32>,

/// 
    #[serde(rename = "ThreadsPerHost")]
    pub threads_per_host: Option<u32>,
}

impl __ProviderHostQuotaConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __SystemClass::new(),
            handles_per_host: None,
            memory_all_hosts: None,
            memory_per_host: None,
            process_limit_all_hosts: None,
            threads_per_host: None,
        }
    }


    /// Sets the value of HandlesPerHost
    pub fn set_handles_per_host(&mut self, value: u32) {
        self.handles_per_host = Some(value);
    }

    /// Gets the value of HandlesPerHost
    pub fn get_handles_per_host(&self) -> Option<&u32> {
        self.handles_per_host.as_ref()
    }

    /// Sets the value of MemoryAllHosts
    pub fn set_memory_all_hosts(&mut self, value: u64) {
        self.memory_all_hosts = Some(value);
    }

    /// Gets the value of MemoryAllHosts
    pub fn get_memory_all_hosts(&self) -> Option<&u64> {
        self.memory_all_hosts.as_ref()
    }

    /// Sets the value of MemoryPerHost
    pub fn set_memory_per_host(&mut self, value: u64) {
        self.memory_per_host = Some(value);
    }

    /// Gets the value of MemoryPerHost
    pub fn get_memory_per_host(&self) -> Option<&u64> {
        self.memory_per_host.as_ref()
    }

    /// Sets the value of ProcessLimitAllHosts
    pub fn set_process_limit_all_hosts(&mut self, value: u32) {
        self.process_limit_all_hosts = Some(value);
    }

    /// Gets the value of ProcessLimitAllHosts
    pub fn get_process_limit_all_hosts(&self) -> Option<&u32> {
        self.process_limit_all_hosts.as_ref()
    }

    /// Sets the value of ThreadsPerHost
    pub fn set_threads_per_host(&mut self, value: u32) {
        self.threads_per_host = Some(value);
    }

    /// Gets the value of ThreadsPerHost
    pub fn get_threads_per_host(&self) -> Option<&u32> {
        self.threads_per_host.as_ref()
    }
}

