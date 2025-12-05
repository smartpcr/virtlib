// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageJob struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageJob {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// 
    #[serde(rename = "BytesProcessed")]
    pub bytes_processed: Option<u64>,

/// 
    #[serde(rename = "BytesTotal")]
    pub bytes_total: Option<u64>,

/// 
    #[serde(rename = "DeleteOnCompletion")]
    pub delete_on_completion: Option<bool>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

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
    #[serde(rename = "IsBackgroundTask")]
    pub is_background_task: Option<bool>,

/// 
    #[serde(rename = "JobState")]
    pub job_state: Option<u16>,

/// 
    #[serde(rename = "JobStatus")]
    pub job_status: Option<String>,

/// 
    #[serde(rename = "LocalOrUtcTime")]
    pub local_or_utc_time: Option<u16>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<u16>,

/// 
    #[serde(rename = "OtherRecoveryAction")]
    pub other_recovery_action: Option<String>,

/// 
    #[serde(rename = "PercentComplete")]
    pub percent_complete: Option<u16>,

/// 
    #[serde(rename = "RecoveryAction")]
    pub recovery_action: Option<u16>,

/// 
    #[serde(rename = "StartTime")]
    pub start_time: Option<String>,

/// 
    #[serde(rename = "StatusDescriptions")]
    pub status_descriptions: Vec<String>,

/// 
    #[serde(rename = "TimeBeforeRemoval")]
    pub time_before_removal: Option<String>,

/// 
    #[serde(rename = "TimeOfLastStateChange")]
    pub time_of_last_state_change: Option<String>,

/// 
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
    pub fn set_job_state(&mut self, value: u16) {
        self.job_state = Some(value);
    }

    /// Gets the value of JobState
    pub fn get_job_state(&self) -> Option<&u16> {
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
    pub fn set_local_or_utc_time(&mut self, value: u16) {
        self.local_or_utc_time = Some(value);
    }

    /// Gets the value of LocalOrUtcTime
    pub fn get_local_or_utc_time(&self) -> Option<&u16> {
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
    pub fn set_operational_status(&mut self, value: Vec<u16>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<u16> {
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
    pub fn set_recovery_action(&mut self, value: u16) {
        self.recovery_action = Some(value);
    }

    /// Gets the value of RecoveryAction
    pub fn get_recovery_action(&self) -> Option<&u16> {
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

/// 

    /// * `requested_state` -  (u16)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn request_state_change(&self, requested_state: u16, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
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

