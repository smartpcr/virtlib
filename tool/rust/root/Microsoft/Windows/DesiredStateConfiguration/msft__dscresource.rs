// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DSCResource struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DSCResource {
    #[serde(flatten)]
    pub base: OMI_BaseResource,

/// 
    #[serde(rename = "DurationInSeconds")]
    pub duration_in_seconds: Option<f64>,

/// 
    #[serde(rename = "Error")]
    pub error: Option<String>,

/// 
    #[serde(rename = "FinalState")]
    pub final_state: Option<String>,

/// 
    #[serde(rename = "InDesiredState")]
    pub in_desired_state: Option<bool>,

/// 
    #[serde(rename = "InitialState")]
    pub initial_state: Option<String>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "RebootRequested")]
    pub reboot_requested: Option<bool>,

/// 
    #[serde(rename = "ResourceName")]
    pub resource_name: Option<String>,

/// 
    #[serde(rename = "StartDate")]
    pub start_date: Option<String>,

/// 
    #[serde(rename = "StateChanged")]
    pub state_changed: Option<bool>,
}

impl MSFT_DSCResource {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: OMI_BaseResource::new(),
            duration_in_seconds: None,
            error: None,
            final_state: None,
            in_desired_state: None,
            initial_state: None,
            instance_name: None,
            reboot_requested: None,
            resource_name: None,
            start_date: None,
            state_changed: None,
        }
    }


    /// Sets the value of DurationInSeconds
    pub fn set_duration_in_seconds(&mut self, value: f64) {
        self.duration_in_seconds = Some(value);
    }

    /// Gets the value of DurationInSeconds
    pub fn get_duration_in_seconds(&self) -> Option<&f64> {
        self.duration_in_seconds.as_ref()
    }

    /// Sets the value of Error
    pub fn set_error(&mut self, value: String) {
        self.error = Some(value);
    }

    /// Gets the value of Error
    pub fn get_error(&self) -> Option<&String> {
        self.error.as_ref()
    }

    /// Sets the value of FinalState
    pub fn set_final_state(&mut self, value: String) {
        self.final_state = Some(value);
    }

    /// Gets the value of FinalState
    pub fn get_final_state(&self) -> Option<&String> {
        self.final_state.as_ref()
    }

    /// Sets the value of InDesiredState
    pub fn set_in_desired_state(&mut self, value: bool) {
        self.in_desired_state = Some(value);
    }

    /// Gets the value of InDesiredState
    pub fn get_in_desired_state(&self) -> Option<&bool> {
        self.in_desired_state.as_ref()
    }

    /// Sets the value of InitialState
    pub fn set_initial_state(&mut self, value: String) {
        self.initial_state = Some(value);
    }

    /// Gets the value of InitialState
    pub fn get_initial_state(&self) -> Option<&String> {
        self.initial_state.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of RebootRequested
    pub fn set_reboot_requested(&mut self, value: bool) {
        self.reboot_requested = Some(value);
    }

    /// Gets the value of RebootRequested
    pub fn get_reboot_requested(&self) -> Option<&bool> {
        self.reboot_requested.as_ref()
    }

    /// Sets the value of ResourceName
    pub fn set_resource_name(&mut self, value: String) {
        self.resource_name = Some(value);
    }

    /// Gets the value of ResourceName
    pub fn get_resource_name(&self) -> Option<&String> {
        self.resource_name.as_ref()
    }

    /// Sets the value of StartDate
    pub fn set_start_date(&mut self, value: String) {
        self.start_date = Some(value);
    }

    /// Gets the value of StartDate
    pub fn get_start_date(&self) -> Option<&String> {
        self.start_date.as_ref()
    }

    /// Sets the value of StateChanged
    pub fn set_state_changed(&mut self, value: bool) {
        self.state_changed = Some(value);
    }

    /// Gets the value of StateChanged
    pub fn get_state_changed(&self) -> Option<&bool> {
        self.state_changed.as_ref()
    }
}

