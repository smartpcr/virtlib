// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Job struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Job {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// Indicates whether or not the job should be automatically deleted upon completion. Note that the 'completion' of a recurring job is defined by its JobRunTimes or UntilTime properties, or when the Job is terminated by manual intervention. If this property is set to false and the job completes, then the extrinsic method DeleteInstance must be used to delete the job instead of updating this property.
    #[serde(rename = "DeleteOnCompletion")]
    pub delete_on_completion: Option<bool>,

/// The time interval that the Job has been executing or the total execution time if the Job is complete. Note that this property is also present in the JobProcessingStatistics class. This class is necessary to capture the processing information for recurring Jobs, because only the 'last' run time can be stored in this single-valued property.
    #[serde(rename = "ElapsedTime")]
    pub elapsed_time: Option<String>,

/// A vendor-specific error code. The value must be set to zero if the Job completed without error. Note that this property is also present in the JobProcessingStatistics class. This class is necessary to capture the processing information for recurring Jobs, because only the 'last' run error can be stored in this single-valued property.
    #[serde(rename = "ErrorCode")]
    pub error_code: Option<u16>,

/// A free-form string that contains the vendor error description. Note that this property is also present in the JobProcessingStatistics class. This class is necessary to capture the processing information for recurring Jobs, because only the 'last' run error can be stored in this single-valued property.
    #[serde(rename = "ErrorDescription")]
    pub error_description: Option<String>,

/// The number of times that the Job should be run. A value of 1 indicates that the Job is not recurring, while any non-zero value indicates a limit to the number of times that the Job will recur. Zero indicates that there is no limit to the number of times that the Job can be processed, but that it is terminated either after the UntilTime or by manual intervention. By default, a Job is processed once.
    #[serde(rename = "JobRunTimes")]
    pub job_run_times: Option<u32>,

/// A free-form string that represents the status of the job. The primary status is reflected in the inherited OperationalStatus property. JobStatus provides additional, implementation-specific details.
    #[serde(rename = "JobStatus")]
    pub job_status: Option<String>,

/// This property indicates whether the times represented in the RunStartInterval and UntilTime properties represent local times or UTC times. Time values are synchronized worldwide by using the enumeration value 2, "UTC Time".
    #[serde(rename = "LocalOrUtcTime")]
    pub local_or_utc_time: Option<Job_LocalOrUtcTime>,

/// The User who is to be notified upon the Job completion or failure.
    #[serde(rename = "Notify")]
    pub notify: Option<String>,

/// A string describing the recovery action when the RecoveryAction property of the instance is 1 ("Other").
    #[serde(rename = "OtherRecoveryAction")]
    pub other_recovery_action: Option<String>,

/// The User that submitted the Job, or the Service or method name that caused the job to be created.
    #[serde(rename = "Owner")]
    pub owner: Option<String>,

/// The percentage of the job that has completed at the time that this value is requested. Note that this property is also present in the JobProcessingStatistics class. This class is necessary to capture the processing information for recurring Jobs, because only the 'last' run data can be stored in this single-valued property. 
/// Note that the value 101 is undefined and will be not be allowed in the next major revision of the specification.
    #[serde(rename = "PercentComplete")]
    pub percent_complete: Option<u16>,

/// Indicates the urgency or importance of execution of the Job. The lower the number, the higher the priority. Note that this property is also present in the JobProcessingStatistics class. This class is necessary to capture the setting information that would influence the results of a job.
    #[serde(rename = "Priority")]
    pub priority: Option<u32>,

/// Describes the recovery action to be taken for an unsuccessfully run Job. The possible values are: 
/// 0 = "Unknown", meaning it is unknown as to what recovery action to take 
/// 1 = "Other", indicating that the recovery action will be specified in the OtherRecoveryAction property 
/// 2 = "Do Not Continue", meaning stop the execution of the job and appropriately update its status 
/// 3 = "Continue With Next Job", meaning continue with the next job in the queue 
/// 4 = "Re-run Job", indicating that the job should be re-run 
/// 5 = "Run Recovery Job", meaning run the Job associated using the RecoveryJob relationship. Note that the recovery Job must already be in the queue from which it will run.
    #[serde(rename = "RecoveryAction")]
    pub recovery_action: Option<Job_RecoveryAction>,

/// The day in the month on which the Job should be processed. There are two different interpretations for this property, depending on the value of DayOfWeek. In one case, RunDay defines the day-in-month on which the Job is processed. This interpretation is used when the DayOfWeek is 0. A positive or negative integer indicates whether the RunDay should be calculated from the beginning or end of the month. For example, 5 indicates the fifth day in the RunMonth and -1 indicates the last day in the RunMonth. 
/// 
/// When RunDayOfWeek is not 0, RunDay is the day-in-month on which the Job is processed, defined in conjunction with RunDayOfWeek. For example, if RunDay is 15 and RunDayOfWeek is Saturday, then the Job is processed on the first Saturday on or after the 15th day in the RunMonth (for example, the third Saturday in the month). If RunDay is 20 and RunDayOfWeek is -Saturday, then this indicates the first Saturday on or before the 20th day in the RunMonth. If RunDay is -1 and RunDayOfWeek is -Sunday, then this indicates the last Sunday in the RunMonth.
    #[serde(rename = "RunDay")]
    pub run_day: Option<u8>,

/// A positive or negative integer used in conjunction with RunDay to indicate the day of the week on which the Job is processed. RunDayOfWeek is set to 0 to indicate an exact day of the month, such as March 1. A positive integer (representing Sunday, Monday, ..., Saturday) means that the day of week is found on or after the specified RunDay. A negative integer (representing -Sunday, -Monday, ..., -Saturday) means that the day of week is found on or BEFORE the RunDay.
    #[serde(rename = "RunDayOfWeek")]
    pub run_day_of_week: Option<Job_RunDayOfWeek>,

/// The month during which the Job should be processed. Specify 0 for January, 1 for February, and so on.
    #[serde(rename = "RunMonth")]
    pub run_month: Option<Job_RunMonth>,

/// The time interval after midnight when the Job should be processed. For example, 
/// 00000000020000.000000:000 
/// indicates that the Job should be run on or after two o'clock, local time or UTC time (distinguished using the LocalOrUtcTime property.
    #[serde(rename = "RunStartInterval")]
    pub run_start_interval: Option<String>,

/// The time that the current Job is scheduled to start. This time can be represented by the actual date and time, or an interval relative to the time that this property is requested. A value of all zeroes indicates that the Job is already executing. The property is deprecated in lieu of the more expressive scheduling properties, RunMonth, RunDay, RunDayOfWeek, and RunStartInterval.
    #[serde(rename = "ScheduledStartTime")]
    pub scheduled_start_time: Option<String>,

/// The time that the Job was actually started. This time can be represented by an actual date and time, or by an interval relative to the time that this property is requested. Note that this property is also present in the JobProcessingStatistics class. This class is necessary to capture the processing information for recurring Jobs, because only the 'last' run time can be stored in this single-valued property.
    #[serde(rename = "StartTime")]
    pub start_time: Option<String>,

/// The time that the Job was submitted to execute. A value of all zeroes indicates that the owning element is not capable of reporting a date and time. Therefore, the ScheduledStartTime and StartTime are reported as intervals relative to the time their values are requested.
    #[serde(rename = "TimeSubmitted")]
    pub time_submitted: Option<String>,

/// The time after which the Job is invalid or should be stopped. This time can be represented by an actual date and time, or by an interval relative to the time that this property is requested. A value of all nines indicates that the Job can run indefinitely.
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
    pub fn set_local_or_utc_time(&mut self, value: Job_LocalOrUtcTime) {
        self.local_or_utc_time = Some(value);
    }

    /// Gets the value of LocalOrUtcTime
    pub fn get_local_or_utc_time(&self) -> Option<&Job_LocalOrUtcTime> {
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
    pub fn set_recovery_action(&mut self, value: Job_RecoveryAction) {
        self.recovery_action = Some(value);
    }

    /// Gets the value of RecoveryAction
    pub fn get_recovery_action(&self) -> Option<&Job_RecoveryAction> {
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
    pub fn set_run_day_of_week(&mut self, value: Job_RunDayOfWeek) {
        self.run_day_of_week = Some(value);
    }

    /// Gets the value of RunDayOfWeek
    pub fn get_run_day_of_week(&self) -> Option<&Job_RunDayOfWeek> {
        self.run_day_of_week.as_ref()
    }

    /// Sets the value of RunMonth
    pub fn set_run_month(&mut self, value: Job_RunMonth) {
        self.run_month = Some(value);
    }

    /// Gets the value of RunMonth
    pub fn get_run_month(&self) -> Option<&Job_RunMonth> {
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

/// KillJob is being deprecated because there is no distinction made between an orderly shutdown and an immediate kill. CIM_ConcreteJob.RequestStateChange() provides 'Terminate' and 'Kill' options to allow this distinction. 
/// A method to kill this job and any underlying processes, and to remove any 'dangling' associations.

    /// * `delete_on_kill` - Indicates whether or not the Job should be automatically deleted upon termination. This parameter takes precedence over the property, DeleteOnCompletion. (bool)

    /// * `return_value` -  (u32)
    pub fn kill_job(&self, delete_on_kill: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DeleteOnKill".to_string(), value: delete_on_kill.into() });
        self.invoke_method("KillJob", &args)

    }

}

