// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ConcreteJob struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ConcreteJob {
    #[serde(flatten)]
    pub base: CIM_Job,

/// JobState is an integer enumeration that indicates the operational state of a Job. It can also indicate transitions between these states, for example, 'Shutting Down' and 'Starting'. Following is a brief description of the states: 
/// New (2) indicates that the job has never been started. 
/// Starting (3) indicates that the job is moving from the 'New', 'Suspended', or 'Service' states into the 'Running' state. 
/// Running (4) indicates that the Job is running. 
/// Suspended (5) indicates that the Job is stopped, but can be restarted in a seamless manner. 
/// Shutting Down (6) indicates that the job is moving to a 'Completed', 'Terminated', or 'Killed' state. 
/// Completed (7) indicates that the job has completed normally. 
/// Terminated (8) indicates that the job has been stopped by a 'Terminate' state change request. The job and all its underlying processes are ended and can be restarted (this is job-specific) only as a new job. 
/// Killed (9) indicates that the job has been stopped by a 'Kill' state change request. Underlying processes might have been left running, and cleanup might be required to free up resources. 
/// Exception (10) indicates that the Job is in an abnormal state that might be indicative of an error condition. Actual status might be displayed though job-specific objects. 
/// Service (11) indicates that the Job is in a vendor-specific state that supports problem discovery, or resolution, or both.
/// Query pending (12) waiting for a client to resolve a query
    #[serde(rename = "JobState")]
    pub job_state: Option<ConcreteJob_JobState>,

/// The amount of time that the Job is retained after it has finished executing, either succeeding or failing in that execution. The job must remain in existence for some period of time regardless of the value of the DeleteOnCompletion property. 
/// The default is five minutes.
    #[serde(rename = "TimeBeforeRemoval")]
    pub time_before_removal: Option<String>,

/// The date or time when the state of the Job last changed. If the state of the Job has not changed and this property is populated, then it must be set to a 0 interval value. If a state change was requested, but rejected or not yet processed, the property must not be updated.
    #[serde(rename = "TimeOfLastStateChange")]
    pub time_of_last_state_change: Option<String>,
}

impl CIM_ConcreteJob {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Job::new(),
            job_state: None,
            time_before_removal: None,
            time_of_last_state_change: None,
        }
    }


    /// Sets the value of JobState
    pub fn set_job_state(&mut self, value: ConcreteJob_JobState) {
        self.job_state = Some(value);
    }

    /// Gets the value of JobState
    pub fn get_job_state(&self) -> Option<&ConcreteJob_JobState> {
        self.job_state.as_ref()
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

/// Requests that the state of the job be changed to the value specified in the RequestedState parameter. Invoking the RequestStateChange method multiple times could result in earlier requests being overwritten or lost. 
/// If 0 is returned, then the task completed successfully. Any other return code indicates an error condition.

    /// * `requested_state` - RequestStateChange changes the state of a job. The possible values are as follows:  Start (2) changes the state to 'Running'.  Suspend (3) stops the job temporarily. The intention is to subsequently restart the job with 'Start'. It might be possible to enter the 'Service' state while suspended. (This is job-specific.)  Terminate (4) stops the job cleanly, saving data, preserving the state, and shutting down all underlying processes in an orderly manner.  Kill (5) terminates the job immediately with no requirement to save data or preserve the state.  Service (6) puts the job into a vendor-specific service state. It might be possible to restart the job. (ConcreteJob_RequestedState)
    /// * `timeout_period` - A timeout period that specifies the maximum amount of time that the client expects the transition to the new state to take. The interval format must be used to specify the TimeoutPeriod. A value of 0 or a null parameter indicates that the client has no time requirements for the transition.  If this property does not contain 0 or null and the implementation does not support this parameter, a return code of 'Use Of Timeout Parameter Not Supported' must be returned. (String)

    /// * `return_value` -  (u32)
    pub fn request_state_change(&self, requested_state: ConcreteJob_RequestedState, timeout_period: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RequestedState".to_string(), value: requested_state.into() });
        args.push(MethodParameter { name: "TimeoutPeriod".to_string(), value: timeout_period.into() });
        self.invoke_method("RequestStateChange", &args)

    }


/// When the job is executing or has terminated without error, then this method returns no CIM_Error instance. However, if the job has failed because of some internal problem or because the job has been terminated by a client, then a CIM_Error instance is returned.

    /// * `error` - If the OperationalStatus on the Job is not "OK", then this method will return a CIM Error instance. Otherwise, when the Job is "OK", null is returned. (String)
    /// * `return_value` -  (u32)
    pub fn get_error(&self, error: &mut String) -> Result<(), WmiError> {

        let result = self.invoke_method("GetError", &[])?;
        let error = result.get_value("Error")?;
        Ok(result.return_value)

    }

}

