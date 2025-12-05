// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageJob struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageJob {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// Indicates the number of bytes processed by this job so far.
    #[serde(rename = "BytesProcessed")]
    pub bytes_processed: Option<u64>,

/// Indicates the total number of bytes being processed by this job.
    #[serde(rename = "BytesTotal")]
    pub bytes_total: Option<u64>,

/// If TRUE, the storage job will be automatically deleted after a short time interval.
    #[serde(rename = "DeleteOnCompletion")]
    pub delete_on_completion: Option<bool>,

/// The Description property provides a textual description of the storage job operation.
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// The time interval that the job has been executing or the total execution time if the storage job is complete.
    #[serde(rename = "ElapsedTime")]
    pub elapsed_time: Option<String>,

/// If the operation that this storage job was tracking has failed, the provider will set this with an error code defined by the method that invoked the operation. If this job tracked a background task, the error code can be set to any valid Storage Management error code as defined in the value map below. If there was no error, this property must be set to 0 - 'Success'. This property should be NULL until the operation has completed.
    #[serde(rename = "ErrorCode")]
    pub error_code: Option<u16>,

/// A free-form string that contains the vendor error description.
    #[serde(rename = "ErrorDescription")]
    pub error_description: Option<String>,

/// If TRUE, this storage job represents an automated background task initiated by the storage subsystem. For all user / management initiated operations, this value should be set to FALSE.
    #[serde(rename = "IsBackgroundTask")]
    pub is_background_task: Option<bool>,

/// The current execution state of the storage job.
    #[serde(rename = "JobState")]
    pub job_state: Option<StorageJob_JobState>,

/// A free-form string that represents the status of the job. The primary status is reflected in the inherited OperationalStatus property. JobStatus provides additional, implementation-specific details.
    #[serde(rename = "JobStatus")]
    pub job_status: Option<String>,

/// This property indicates whether the times represented in the StartTime, TimeOfLastStateChange, and TimeSubmitted properties represent local times or UTC times. Time values are synchronized worldwide by using the enumeration value 2 - 'UTC Time'.
    #[serde(rename = "LocalOrUtcTime")]
    pub local_or_utc_time: Option<StorageJob_LocalOrUtcTime>,

/// A system defined name for this storage job.
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// Indicates the current statuses of the element.
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<StorageJob_OperationalStatus>,

/// Denotes a vendor-specific recovery action to be taken for an unsuccessfully run job. This value should only be set if RecoveryAction is set to 1 - 'Other'.
    #[serde(rename = "OtherRecoveryAction")]
    pub other_recovery_action: Option<String>,

/// The percentage of the job that has completed at the time that this value is requested.
    #[serde(rename = "PercentComplete")]
    pub percent_complete: Option<u16>,

/// Describes the recovery action to be taken for an unsuccessfully run job. The possible values are: 
/// 0 - 'Unknown' meaning it is unknown as to what recovery action to take 
/// 1 - 'Other' indicating that the recovery action will be specified in the OtherRecoveryAction property 
/// 2 - 'Do Not Continue' meaning stop the execution of the job and appropriately update its status 
/// 3 - 'Continue With Next Job' meaning continue with the next job in the queue 
/// 4 - 'Re-run Job' indicating that the job should be re-run 
/// 
    #[serde(rename = "RecoveryAction")]
    pub recovery_action: Option<StorageJob_RecoveryAction>,

/// The time that the job was actually started.
    #[serde(rename = "StartTime")]
    pub start_time: Option<String>,

/// Strings describing the various OperationalStatus array values. For example, if "Stopping" is the value assigned to OperationalStatus, this property may contain an explanation as to why an object is being stopped. Note that entries in this array are correlated with those at the same array index in OperationalStatus.
    #[serde(rename = "StatusDescriptions")]
    pub status_descriptions: Vec<String>,

/// The amount of time that the Job is retained after it has finished executing, regardless of whether it failed during execution. The job must remain in existence for some period of time regardless of the value of the DeleteOnCompletion property. 
/// 
    #[serde(rename = "TimeBeforeRemoval")]
    pub time_before_removal: Option<String>,

/// The date or time when the state of the job last changed. If the state of the job has not changed and this property is populated, it must be set to a 0 interval value. If a state change was requested, but was rejected or not yet processed, the property must not be updated.
    #[serde(rename = "TimeOfLastStateChange")]
    pub time_of_last_state_change: Option<String>,

/// The time that the job was submitted to execute. A value of all zeroes indicates that the owning element is not capable of reporting a date and time.
    #[serde(rename = "TimeSubmitted")]
    pub time_submitted: Option<String>,
}

impl MSFT_StorageJob {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            bytes_processed: None,
            bytes_total: None,
            delete_on_completion: None,
            description: None,
            elapsed_time: None,
            error_code: None,
            error_description: None,
            is_background_task: None,
            job_state: None,
            job_status: None,
            local_or_utc_time: None,
            name: None,
            operational_status: Vec::new(),
            other_recovery_action: None,
            percent_complete: None,
            recovery_action: None,
            start_time: None,
            status_descriptions: Vec::new(),
            time_before_removal: None,
            time_of_last_state_change: None,
            time_submitted: None,
        }
    }


    /// Sets the value of BytesProcessed
    pub fn set_bytes_processed(&mut self, value: u64) {
        self.bytes_processed = Some(value);
    }

    /// Gets the value of BytesProcessed
    pub fn get_bytes_processed(&self) -> Option<&u64> {
        self.bytes_processed.as_ref()
    }

    /// Sets the value of BytesTotal
    pub fn set_bytes_total(&mut self, value: u64) {
        self.bytes_total = Some(value);
    }

    /// Gets the value of BytesTotal
    pub fn get_bytes_total(&self) -> Option<&u64> {
        self.bytes_total.as_ref()
    }

    /// Sets the value of DeleteOnCompletion
    pub fn set_delete_on_completion(&mut self, value: bool) {
        self.delete_on_completion = Some(value);
    }

    /// Gets the value of DeleteOnCompletion
    pub fn get_delete_on_completion(&self) -> Option<&bool> {
        self.delete_on_completion.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
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

    /// Sets the value of IsBackgroundTask
    pub fn set_is_background_task(&mut self, value: bool) {
        self.is_background_task = Some(value);
    }

    /// Gets the value of IsBackgroundTask
    pub fn get_is_background_task(&self) -> Option<&bool> {
        self.is_background_task.as_ref()
    }

    /// Sets the value of JobState
    pub fn set_job_state(&mut self, value: StorageJob_JobState) {
        self.job_state = Some(value);
    }

    /// Gets the value of JobState
    pub fn get_job_state(&self) -> Option<&StorageJob_JobState> {
        self.job_state.as_ref()
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
    pub fn set_local_or_utc_time(&mut self, value: StorageJob_LocalOrUtcTime) {
        self.local_or_utc_time = Some(value);
    }

    /// Gets the value of LocalOrUtcTime
    pub fn get_local_or_utc_time(&self) -> Option<&StorageJob_LocalOrUtcTime> {
        self.local_or_utc_time.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of OperationalStatus
    pub fn set_operational_status(&mut self, value: Vec<StorageJob_OperationalStatus>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<StorageJob_OperationalStatus> {
        &self.operational_status
    }

    /// Sets the value of OtherRecoveryAction
    pub fn set_other_recovery_action(&mut self, value: String) {
        self.other_recovery_action = Some(value);
    }

    /// Gets the value of OtherRecoveryAction
    pub fn get_other_recovery_action(&self) -> Option<&String> {
        self.other_recovery_action.as_ref()
    }

    /// Sets the value of PercentComplete
    pub fn set_percent_complete(&mut self, value: u16) {
        self.percent_complete = Some(value);
    }

    /// Gets the value of PercentComplete
    pub fn get_percent_complete(&self) -> Option<&u16> {
        self.percent_complete.as_ref()
    }

    /// Sets the value of RecoveryAction
    pub fn set_recovery_action(&mut self, value: StorageJob_RecoveryAction) {
        self.recovery_action = Some(value);
    }

    /// Gets the value of RecoveryAction
    pub fn get_recovery_action(&self) -> Option<&StorageJob_RecoveryAction> {
        self.recovery_action.as_ref()
    }

    /// Sets the value of StartTime
    pub fn set_start_time(&mut self, value: String) {
        self.start_time = Some(value);
    }

    /// Gets the value of StartTime
    pub fn get_start_time(&self) -> Option<&String> {
        self.start_time.as_ref()
    }

    /// Sets the value of StatusDescriptions
    pub fn set_status_descriptions(&mut self, value: Vec<String>) {
        self.status_descriptions = value;
    }

    /// Gets the value of StatusDescriptions
    pub fn get_status_descriptions(&self) -> &Vec<String> {
        &self.status_descriptions
    }

    /// Sets the value of TimeBeforeRemoval
    pub fn set_time_before_removal(&mut self, value: String) {
        self.time_before_removal = Some(value);
    }

    /// Gets the value of TimeBeforeRemoval
    pub fn get_time_before_removal(&self) -> Option<&String> {
        self.time_before_removal.as_ref()
    }

    /// Sets the value of TimeOfLastStateChange
    pub fn set_time_of_last_state_change(&mut self, value: String) {
        self.time_of_last_state_change = Some(value);
    }

    /// Gets the value of TimeOfLastStateChange
    pub fn get_time_of_last_state_change(&self) -> Option<&String> {
        self.time_of_last_state_change.as_ref()
    }

    /// Sets the value of TimeSubmitted
    pub fn set_time_submitted(&mut self, value: String) {
        self.time_submitted = Some(value);
    }

    /// Gets the value of TimeSubmitted
    pub fn get_time_submitted(&self) -> Option<&String> {
        self.time_submitted.as_ref()
    }

/// Requests that the state of the job be changed to the value specified in the RequestedState parameter. Invoking the RequestStateChange method multiple times could result in earlier requests being overwritten or lost.

    /// * `requested_state` - RequestStateChange changes the state of a job. The possible values are as follows:  2 - 'Start' changes the state to 'Running'.  3 - 'Suspend' stops the job temporarily. The intention is to subsequently restart the job with a second call to RequestStateChange requesting 1 - 'Start'. It might be possible to enter the 'Service' state while suspended. (This is job-specific.)  4 - 'Terminate' stops the job cleanly, saving data, preserving the state, and shutting down all underlying processes in an orderly manner.  5 - 'Kill' terminates the job immediately with no requirement to save data or preserve the state.  6 - 'Service' puts the job into a vendor-specific service state. It might be possible to restart the job. (StorageJob_RequestedState)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn request_state_change(&self, requested_state: StorageJob_RequestedState, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RequestedState".to_string(), value: requested_state.into() });

        let result = self.invoke_method("RequestStateChange", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn get_extended_status(&self, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("GetExtendedStatus", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `channels` -  (u16[])
    /// * `messages` -  (String[])
    /// * `return_value` -  (u32)
    pub fn get_messages(&self, channels: &mut Vec<u16>, messages: &mut Vec<String>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetMessages", &[])?;
        let channels = result.get_value("Channels")?;
        let messages = result.get_value("Messages")?;
        Ok(result.return_value)

    }


/// 

    /// * `out_parameters` -  (MSFT_StorageJobOutParams)
    /// * `return_value` -  (u32)
    pub fn get_out_parameters(&self, out_parameters: &mut MSFT_StorageJobOutParams) -> Result<(), WmiError> {

        let result = self.invoke_method("GetOutParameters", &[])?;
        let out_parameters = result.get_value("OutParameters")?;
        Ok(result.return_value)

    }

}

