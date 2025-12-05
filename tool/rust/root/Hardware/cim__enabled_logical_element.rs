// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Hardware
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;
use Microsoft.Test.Wmi.root.StandardCimv2;


/// CIM_EnabledLogicalElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_EnabledLogicalElement {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "EnabledDefault")]
    pub enabled_default: Option<u16>,

/// 
    #[serde(rename = "EnabledState")]
    pub enabled_state: Option<u16>,

/// 
    #[serde(rename = "OtherEnabledState")]
    pub other_enabled_state: Option<String>,

/// 
    #[serde(rename = "RequestedState")]
    pub requested_state: Option<u16>,

/// 
    #[serde(rename = "TimeOfLastStateChange")]
    pub time_of_last_state_change: Option<String>,
}

impl CIM_EnabledLogicalElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            enabled_default: None,
            enabled_state: None,
            other_enabled_state: None,
            requested_state: None,
            time_of_last_state_change: None,
        }
    }


    /// Sets the value of EnabledDefault
    pub fn set_enabled_default(&mut self, value: u16) {
        self.enabled_default = Some(value);
    }

    /// Gets the value of EnabledDefault
    pub fn get_enabled_default(&self) -> Option<&u16> {
        self.enabled_default.as_ref()
    }

    /// Sets the value of EnabledState
    pub fn set_enabled_state(&mut self, value: u16) {
        self.enabled_state = Some(value);
    }

    /// Gets the value of EnabledState
    pub fn get_enabled_state(&self) -> Option<&u16> {
        self.enabled_state.as_ref()
    }

    /// Sets the value of OtherEnabledState
    pub fn set_other_enabled_state(&mut self, value: String) {
        self.other_enabled_state = Some(value);
    }

    /// Gets the value of OtherEnabledState
    pub fn get_other_enabled_state(&self) -> Option<&String> {
        self.other_enabled_state.as_ref()
    }

    /// Sets the value of RequestedState
    pub fn set_requested_state(&mut self, value: u16) {
        self.requested_state = Some(value);
    }

    /// Gets the value of RequestedState
    pub fn get_requested_state(&self) -> Option<&u16> {
        self.requested_state.as_ref()
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

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn request_state_change(&self, requested_state: u16, job: &mut CIM_ConcreteJob, timeout_period: &Option<String>, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RequestedState".to_string(), value: requested_state.into() });
        if let Some(val) = timeout_period {
            args.push(MethodParameter { name: "TimeoutPeriod".to_string(), value: val.into() });
        }

        let result = self.invoke_method_with_job("RequestStateChange", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }

}

