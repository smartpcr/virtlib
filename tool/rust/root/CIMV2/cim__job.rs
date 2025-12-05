// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Job struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Job {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "ElapsedTime")]
    pub elapsed_time: Option<String>,

/// 
    #[serde(rename = "JobStatus")]
    pub job_status: Option<String>,

/// 
    #[serde(rename = "Notify")]
    pub notify: Option<String>,

/// 
    #[serde(rename = "Owner")]
    pub owner: Option<String>,

/// 
    #[serde(rename = "Priority")]
    pub priority: Option<u32>,

/// 
    #[serde(rename = "StartTime")]
    pub start_time: Option<String>,

/// 
    #[serde(rename = "TimeSubmitted")]
    pub time_submitted: Option<String>,

/// 
    #[serde(rename = "UntilTime")]
    pub until_time: Option<String>,
}

impl CIM_Job {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            elapsed_time: None,
            job_status: None,
            notify: None,
            owner: None,
            priority: None,
            start_time: None,
            time_submitted: None,
            until_time: None,
        }
    }


    /// Sets the value of ElapsedTime
    pub fn set_elapsed_time(&mut self, value: String) {
        self.elapsed_time = Some(value);
    }

    /// Gets the value of ElapsedTime
    pub fn get_elapsed_time(&self) -> Option<&String> {
        self.elapsed_time.as_ref()
    }

    /// Sets the value of JobStatus
    pub fn set_job_status(&mut self, value: String) {
        self.job_status = Some(value);
    }

    /// Gets the value of JobStatus
    pub fn get_job_status(&self) -> Option<&String> {
        self.job_status.as_ref()
    }

    /// Sets the value of Notify
    pub fn set_notify(&mut self, value: String) {
        self.notify = Some(value);
    }

    /// Gets the value of Notify
    pub fn get_notify(&self) -> Option<&String> {
        self.notify.as_ref()
    }

    /// Sets the value of Owner
    pub fn set_owner(&mut self, value: String) {
        self.owner = Some(value);
    }

    /// Gets the value of Owner
    pub fn get_owner(&self) -> Option<&String> {
        self.owner.as_ref()
    }

    /// Sets the value of Priority
    pub fn set_priority(&mut self, value: u32) {
        self.priority = Some(value);
    }

    /// Gets the value of Priority
    pub fn get_priority(&self) -> Option<&u32> {
        self.priority.as_ref()
    }

    /// Sets the value of StartTime
    pub fn set_start_time(&mut self, value: String) {
        self.start_time = Some(value);
    }

    /// Gets the value of StartTime
    pub fn get_start_time(&self) -> Option<&String> {
        self.start_time.as_ref()
    }

    /// Sets the value of TimeSubmitted
    pub fn set_time_submitted(&mut self, value: String) {
        self.time_submitted = Some(value);
    }

    /// Gets the value of TimeSubmitted
    pub fn get_time_submitted(&self) -> Option<&String> {
        self.time_submitted.as_ref()
    }

    /// Sets the value of UntilTime
    pub fn set_until_time(&mut self, value: String) {
        self.until_time = Some(value);
    }

    /// Gets the value of UntilTime
    pub fn get_until_time(&self) -> Option<&String> {
        self.until_time.as_ref()
    }
}

