// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.HardwareManagement
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Job struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Job {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "DeleteOnCompletion")]
    pub delete_on_completion: Option<bool>,

/// 
    #[serde(rename = "ElapsedTime")]
    pub elapsed_time: Option<String>,

/// 
    #[serde(rename = "ErrorCode")]
    pub error_code: Option<u16>,

/// 
    #[serde(rename = "ErrorDescription")]
    pub error_description: Option<String>,

/// 
    #[serde(rename = "JobRunTimes")]
    pub job_run_times: Option<u32>,

/// 
    #[serde(rename = "JobStatus")]
    pub job_status: Option<String>,

/// 
    #[serde(rename = "LocalOrUtcTime")]
    pub local_or_utc_time: Option<u16>,

/// 
    #[serde(rename = "Notify")]
    pub notify: Option<String>,

/// 
    #[serde(rename = "OtherRecoveryAction")]
    pub other_recovery_action: Option<String>,

/// 
    #[serde(rename = "Owner")]
    pub owner: Option<String>,

/// 
    #[serde(rename = "PercentComplete")]
    pub percent_complete: Option<u16>,

/// 
    #[serde(rename = "Priority")]
    pub priority: Option<u32>,

/// 
    #[serde(rename = "RecoveryAction")]
    pub recovery_action: Option<u16>,

/// 
    #[serde(rename = "RunDay")]
    pub run_day: Option<u8>,

/// 
    #[serde(rename = "RunDayOfWeek")]
    pub run_day_of_week: Option<u8>,

/// 
    #[serde(rename = "RunMonth")]
    pub run_month: Option<u8>,

/// 
    #[serde(rename = "RunStartInterval")]
    pub run_start_interval: Option<String>,

/// 
    #[serde(rename = "ScheduledStartTime")]
    pub scheduled_start_time: Option<String>,

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
            delete_on_completion: None,
            elapsed_time: None,
            error_code: None,
            error_description: None,
            job_run_times: None,
            job_status: None,
            local_or_utc_time: None,
            notify: None,
            other_recovery_action: None,
            owner: None,
            percent_complete: None,
            priority: None,
            recovery_action: None,
            run_day: None,
            run_day_of_week: None,
            run_month: None,
            run_start_interval: None,
            scheduled_start_time: None,
            start_time: None,
            time_submitted: None,
            until_time: None,
        }
    }


    /// Sets the value of DeleteOnCompletion
    pub fn set_delete_on_completion(&mut self, value: bool) {
        self.delete_on_completion = Some(value);
    }

    /// Gets the value of DeleteOnCompletion
    pub fn get_delete_on_completion(&self) -> Option<&bool> {
        self.delete_on_completion.as_ref()
    }

    /// Sets the value of ElapsedTime
    pub fn set_elapsed_time(&mut self, value: String) {
        self.elapsed_time = Some(value);
    }

    /// Gets the value of ElapsedTime
    pub fn get_elapsed_time(&self) -> Option<&String> {
        self.elapsed_time.as_ref()
    }

    /// Sets the value of ErrorCode
    pub fn set_error_code(&mut self, value: u16) {
        self.error_code = Some(value);
    }

    /// Gets the value of ErrorCode
    pub fn get_error_code(&self) -> Option<&u16> {
        self.error_code.as_ref()
    }

    /// Sets the value of ErrorDescription
    pub fn set_error_description(&mut self, value: String) {
        self.error_description = Some(value);
    }

    /// Gets the value of ErrorDescription
    pub fn get_error_description(&self) -> Option<&String> {
        self.error_description.as_ref()
    }

    /// Sets the value of JobRunTimes
    pub fn set_job_run_times(&mut self, value: u32) {
        self.job_run_times = Some(value);
    }

    /// Gets the value of JobRunTimes
    pub fn get_job_run_times(&self) -> Option<&u32> {
        self.job_run_times.as_ref()
    }

    /// Sets the value of JobStatus
    pub fn set_job_status(&mut self, value: String) {
        self.job_status = Some(value);
    }

    /// Gets the value of JobStatus
    pub fn get_job_status(&self) -> Option<&String> {
        self.job_status.as_ref()
    }

    /// Sets the value of LocalOrUtcTime
    pub fn set_local_or_utc_time(&mut self, value: u16) {
        self.local_or_utc_time = Some(value);
    }

    /// Gets the value of LocalOrUtcTime
    pub fn get_local_or_utc_time(&self) -> Option<&u16> {
        self.local_or_utc_time.as_ref()
    }

    /// Sets the value of Notify
    pub fn set_notify(&mut self, value: String) {
        self.notify = Some(value);
    }

    /// Gets the value of Notify
    pub fn get_notify(&self) -> Option<&String> {
        self.notify.as_ref()
    }

    /// Sets the value of OtherRecoveryAction
    pub fn set_other_recovery_action(&mut self, value: String) {
        self.other_recovery_action = Some(value);
    }

    /// Gets the value of OtherRecoveryAction
    pub fn get_other_recovery_action(&self) -> Option<&String> {
        self.other_recovery_action.as_ref()
    }

    /// Sets the value of Owner
    pub fn set_owner(&mut self, value: String) {
        self.owner = Some(value);
    }

    /// Gets the value of Owner
    pub fn get_owner(&self) -> Option<&String> {
        self.owner.as_ref()
    }

    /// Sets the value of PercentComplete
    pub fn set_percent_complete(&mut self, value: u16) {
        self.percent_complete = Some(value);
    }

    /// Gets the value of PercentComplete
    pub fn get_percent_complete(&self) -> Option<&u16> {
        self.percent_complete.as_ref()
    }

    /// Sets the value of Priority
    pub fn set_priority(&mut self, value: u32) {
        self.priority = Some(value);
    }

    /// Gets the value of Priority
    pub fn get_priority(&self) -> Option<&u32> {
        self.priority.as_ref()
    }

    /// Sets the value of RecoveryAction
    pub fn set_recovery_action(&mut self, value: u16) {
        self.recovery_action = Some(value);
    }

    /// Gets the value of RecoveryAction
    pub fn get_recovery_action(&self) -> Option<&u16> {
        self.recovery_action.as_ref()
    }

    /// Sets the value of RunDay
    pub fn set_run_day(&mut self, value: u8) {
        self.run_day = Some(value);
    }

    /// Gets the value of RunDay
    pub fn get_run_day(&self) -> Option<&u8> {
        self.run_day.as_ref()
    }

    /// Sets the value of RunDayOfWeek
    pub fn set_run_day_of_week(&mut self, value: u8) {
        self.run_day_of_week = Some(value);
    }

    /// Gets the value of RunDayOfWeek
    pub fn get_run_day_of_week(&self) -> Option<&u8> {
        self.run_day_of_week.as_ref()
    }

    /// Sets the value of RunMonth
    pub fn set_run_month(&mut self, value: u8) {
        self.run_month = Some(value);
    }

    /// Gets the value of RunMonth
    pub fn get_run_month(&self) -> Option<&u8> {
        self.run_month.as_ref()
    }

    /// Sets the value of RunStartInterval
    pub fn set_run_start_interval(&mut self, value: String) {
        self.run_start_interval = Some(value);
    }

    /// Gets the value of RunStartInterval
    pub fn get_run_start_interval(&self) -> Option<&String> {
        self.run_start_interval.as_ref()
    }

    /// Sets the value of ScheduledStartTime
    pub fn set_scheduled_start_time(&mut self, value: String) {
        self.scheduled_start_time = Some(value);
    }

    /// Gets the value of ScheduledStartTime
    pub fn get_scheduled_start_time(&self) -> Option<&String> {
        self.scheduled_start_time.as_ref()
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

/// 

    /// * `delete_on_kill` -  (bool)

    /// * `return_value` -  (u32)
    pub fn kill_job(&self, delete_on_kill: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DeleteOnKill".to_string(), value: delete_on_kill.into() });
        self.invoke_method("KillJob", &args)

    }

}

