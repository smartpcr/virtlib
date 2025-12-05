// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_NamedJobObjectLimitSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_NamedJobObjectLimitSetting {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "ActiveProcessLimit")]
    pub active_process_limit: Option<u32>,

/// 
    #[serde(rename = "Affinity")]
    pub affinity: Option<u32>,

/// 
    #[serde(rename = "JobMemoryLimit")]
    pub job_memory_limit: Option<u32>,

/// 
    #[serde(rename = "LimitFlags")]
    pub limit_flags: Option<u32>,

/// 
    #[serde(rename = "MaximumWorkingSetSize")]
    pub maximum_working_set_size: Option<u32>,

/// 
    #[serde(rename = "MinimumWorkingSetSize")]
    pub minimum_working_set_size: Option<u32>,

/// 
    #[serde(rename = "PerJobUserTimeLimit")]
    pub per_job_user_time_limit: Option<u64>,

/// 
    #[serde(rename = "PerProcessUserTimeLimit")]
    pub per_process_user_time_limit: Option<u64>,

/// 
    #[serde(rename = "PriorityClass")]
    pub priority_class: Option<u32>,

/// 
    #[serde(rename = "ProcessMemoryLimit")]
    pub process_memory_limit: Option<u32>,

/// 
    #[serde(rename = "SchedulingClass")]
    pub scheduling_class: Option<u32>,
}

impl Win32_NamedJobObjectLimitSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            active_process_limit: None,
            affinity: None,
            job_memory_limit: None,
            limit_flags: None,
            maximum_working_set_size: None,
            minimum_working_set_size: None,
            per_job_user_time_limit: None,
            per_process_user_time_limit: None,
            priority_class: None,
            process_memory_limit: None,
            scheduling_class: None,
        }
    }


    /// Sets the value of ActiveProcessLimit
    pub fn set_active_process_limit(&mut self, value: u32) {
        self.active_process_limit = Some(value);
    }

    /// Gets the value of ActiveProcessLimit
    pub fn get_active_process_limit(&self) -> Option<&u32> {
        self.active_process_limit.as_ref()
    }

    /// Sets the value of Affinity
    pub fn set_affinity(&mut self, value: u32) {
        self.affinity = Some(value);
    }

    /// Gets the value of Affinity
    pub fn get_affinity(&self) -> Option<&u32> {
        self.affinity.as_ref()
    }

    /// Sets the value of JobMemoryLimit
    pub fn set_job_memory_limit(&mut self, value: u32) {
        self.job_memory_limit = Some(value);
    }

    /// Gets the value of JobMemoryLimit
    pub fn get_job_memory_limit(&self) -> Option<&u32> {
        self.job_memory_limit.as_ref()
    }

    /// Sets the value of LimitFlags
    pub fn set_limit_flags(&mut self, value: u32) {
        self.limit_flags = Some(value);
    }

    /// Gets the value of LimitFlags
    pub fn get_limit_flags(&self) -> Option<&u32> {
        self.limit_flags.as_ref()
    }

    /// Sets the value of MaximumWorkingSetSize
    pub fn set_maximum_working_set_size(&mut self, value: u32) {
        self.maximum_working_set_size = Some(value);
    }

    /// Gets the value of MaximumWorkingSetSize
    pub fn get_maximum_working_set_size(&self) -> Option<&u32> {
        self.maximum_working_set_size.as_ref()
    }

    /// Sets the value of MinimumWorkingSetSize
    pub fn set_minimum_working_set_size(&mut self, value: u32) {
        self.minimum_working_set_size = Some(value);
    }

    /// Gets the value of MinimumWorkingSetSize
    pub fn get_minimum_working_set_size(&self) -> Option<&u32> {
        self.minimum_working_set_size.as_ref()
    }

    /// Sets the value of PerJobUserTimeLimit
    pub fn set_per_job_user_time_limit(&mut self, value: u64) {
        self.per_job_user_time_limit = Some(value);
    }

    /// Gets the value of PerJobUserTimeLimit
    pub fn get_per_job_user_time_limit(&self) -> Option<&u64> {
        self.per_job_user_time_limit.as_ref()
    }

    /// Sets the value of PerProcessUserTimeLimit
    pub fn set_per_process_user_time_limit(&mut self, value: u64) {
        self.per_process_user_time_limit = Some(value);
    }

    /// Gets the value of PerProcessUserTimeLimit
    pub fn get_per_process_user_time_limit(&self) -> Option<&u64> {
        self.per_process_user_time_limit.as_ref()
    }

    /// Sets the value of PriorityClass
    pub fn set_priority_class(&mut self, value: u32) {
        self.priority_class = Some(value);
    }

    /// Gets the value of PriorityClass
    pub fn get_priority_class(&self) -> Option<&u32> {
        self.priority_class.as_ref()
    }

    /// Sets the value of ProcessMemoryLimit
    pub fn set_process_memory_limit(&mut self, value: u32) {
        self.process_memory_limit = Some(value);
    }

    /// Gets the value of ProcessMemoryLimit
    pub fn get_process_memory_limit(&self) -> Option<&u32> {
        self.process_memory_limit.as_ref()
    }

    /// Sets the value of SchedulingClass
    pub fn set_scheduling_class(&mut self, value: u32) {
        self.scheduling_class = Some(value);
    }

    /// Gets the value of SchedulingClass
    pub fn get_scheduling_class(&self) -> Option<&u32> {
        self.scheduling_class.as_ref()
    }
}

