// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ConcreteJob struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ConcreteJob {
    #[serde(flatten)]
    pub base: CIM_Job,

/// 
    #[serde(rename = "JobState")]
    pub job_state: Option<u16>,

/// 
    #[serde(rename = "TimeBeforeRemoval")]
    pub time_before_removal: Option<String>,

/// 
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
    pub fn set_job_state(&mut self, value: u16) {
        self.job_state = Some(value);
    }

    /// Gets the value of JobState
    pub fn get_job_state(&self) -> Option<&u16> {
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

/// 

    /// * `requested_state` -  (u16)
    /// * `timeout_period` -  (String)

    /// * `return_value` -  (u32)
    pub fn request_state_change(&self, requested_state: u16, timeout_period: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RequestedState".to_string(), value: requested_state.into() });
        args.push(MethodParameter { name: "TimeoutPeriod".to_string(), value: timeout_period.into() });
        self.invoke_method("RequestStateChange", &args)

    }


/// 

    /// * `error` -  (CIM_Error)
    /// * `return_value` -  (u32)
    pub fn get_error(&self, error: &mut CIM_Error) -> Result<(), WmiError> {

        let result = self.invoke_method("GetError", &[])?;
        let error = result.get_value("Error")?;
        Ok(result.return_value)

    }

}

