// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_ResumeKeyFilter_ResumeKeyFilter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_ResumeKeyFilter_ResumeKeyFilter {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "CancelledHandleCount")]
    pub cancelled_handle_count: Option<u64>,

/// 
    #[serde(rename = "CurrentActiveHandleCount")]
    pub current_active_handle_count: Option<u64>,

/// 
    #[serde(rename = "CurrentInactiveHandleCount")]
    pub current_inactive_handle_count: Option<u64>,

/// 
    #[serde(rename = "FSFailedResumeHandleCount")]
    pub fsfailed_resume_handle_count: Option<u64>,

/// 
    #[serde(rename = "ReplayedHandleCount")]
    pub replayed_handle_count: Option<u64>,

/// 
    #[serde(rename = "ResumedHandleCount")]
    pub resumed_handle_count: Option<u64>,

/// 
    #[serde(rename = "RKFailedResumeHandleCount")]
    pub rkfailed_resume_handle_count: Option<u64>,

/// 
    #[serde(rename = "SuspendedHandleCount")]
    pub suspended_handle_count: Option<u64>,
}

impl Win32_PerfRawData_ResumeKeyFilter_ResumeKeyFilter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            cancelled_handle_count: None,
            current_active_handle_count: None,
            current_inactive_handle_count: None,
            fsfailed_resume_handle_count: None,
            replayed_handle_count: None,
            resumed_handle_count: None,
            rkfailed_resume_handle_count: None,
            suspended_handle_count: None,
        }
    }


    /// Sets the value of CancelledHandleCount
    pub fn set_cancelled_handle_count(&mut self, value: u64) {
        self.cancelled_handle_count = Some(value);
    }

    /// Gets the value of CancelledHandleCount
    pub fn get_cancelled_handle_count(&self) -> Option<&u64> {
        self.cancelled_handle_count.as_ref()
    }

    /// Sets the value of CurrentActiveHandleCount
    pub fn set_current_active_handle_count(&mut self, value: u64) {
        self.current_active_handle_count = Some(value);
    }

    /// Gets the value of CurrentActiveHandleCount
    pub fn get_current_active_handle_count(&self) -> Option<&u64> {
        self.current_active_handle_count.as_ref()
    }

    /// Sets the value of CurrentInactiveHandleCount
    pub fn set_current_inactive_handle_count(&mut self, value: u64) {
        self.current_inactive_handle_count = Some(value);
    }

    /// Gets the value of CurrentInactiveHandleCount
    pub fn get_current_inactive_handle_count(&self) -> Option<&u64> {
        self.current_inactive_handle_count.as_ref()
    }

    /// Sets the value of FSFailedResumeHandleCount
    pub fn set_fsfailed_resume_handle_count(&mut self, value: u64) {
        self.fsfailed_resume_handle_count = Some(value);
    }

    /// Gets the value of FSFailedResumeHandleCount
    pub fn get_fsfailed_resume_handle_count(&self) -> Option<&u64> {
        self.fsfailed_resume_handle_count.as_ref()
    }

    /// Sets the value of ReplayedHandleCount
    pub fn set_replayed_handle_count(&mut self, value: u64) {
        self.replayed_handle_count = Some(value);
    }

    /// Gets the value of ReplayedHandleCount
    pub fn get_replayed_handle_count(&self) -> Option<&u64> {
        self.replayed_handle_count.as_ref()
    }

    /// Sets the value of ResumedHandleCount
    pub fn set_resumed_handle_count(&mut self, value: u64) {
        self.resumed_handle_count = Some(value);
    }

    /// Gets the value of ResumedHandleCount
    pub fn get_resumed_handle_count(&self) -> Option<&u64> {
        self.resumed_handle_count.as_ref()
    }

    /// Sets the value of RKFailedResumeHandleCount
    pub fn set_rkfailed_resume_handle_count(&mut self, value: u64) {
        self.rkfailed_resume_handle_count = Some(value);
    }

    /// Gets the value of RKFailedResumeHandleCount
    pub fn get_rkfailed_resume_handle_count(&self) -> Option<&u64> {
        self.rkfailed_resume_handle_count.as_ref()
    }

    /// Sets the value of SuspendedHandleCount
    pub fn set_suspended_handle_count(&mut self, value: u64) {
        self.suspended_handle_count = Some(value);
    }

    /// Gets the value of SuspendedHandleCount
    pub fn get_suspended_handle_count(&self) -> Option<&u64> {
        self.suspended_handle_count.as_ref()
    }
}

