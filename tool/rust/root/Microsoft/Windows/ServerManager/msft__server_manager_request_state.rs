// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ServerManager
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ServerManagerRequestState struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ServerManagerRequestState {

/// 
    #[serde(rename = "Error")]
    pub error: Option<MSFT_ServerManagerDeploymentError>,

/// 
    #[serde(rename = "ProgressTicks")]
    pub progress_ticks: Option<u32>,

/// 
    #[serde(rename = "RequestState")]
    pub request_state: Option<u8>,

/// 
    #[serde(rename = "RestartRequired")]
    pub restart_required: Option<bool>,

/// 
    #[serde(rename = "TotalTicks")]
    pub total_ticks: Option<u32>,

/// 
    #[serde(rename = "Warnings")]
    pub warnings: Vec<String>,
}

impl MSFT_ServerManagerRequestState {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            error: None,
            progress_ticks: None,
            request_state: None,
            restart_required: None,
            total_ticks: None,
            warnings: Vec::new(),
        }
    }


    /// Sets the value of Error
    pub fn set_error(&mut self, value: MSFT_ServerManagerDeploymentError) {
        self.error = Some(value);
    }

    /// Gets the value of Error
    pub fn get_error(&self) -> Option<&MSFT_ServerManagerDeploymentError> {
        self.error.as_ref()
    }

    /// Sets the value of ProgressTicks
    pub fn set_progress_ticks(&mut self, value: u32) {
        self.progress_ticks = Some(value);
    }

    /// Gets the value of ProgressTicks
    pub fn get_progress_ticks(&self) -> Option<&u32> {
        self.progress_ticks.as_ref()
    }

    /// Sets the value of RequestState
    pub fn set_request_state(&mut self, value: u8) {
        self.request_state = Some(value);
    }

    /// Gets the value of RequestState
    pub fn get_request_state(&self) -> Option<&u8> {
        self.request_state.as_ref()
    }

    /// Sets the value of RestartRequired
    pub fn set_restart_required(&mut self, value: bool) {
        self.restart_required = Some(value);
    }

    /// Gets the value of RestartRequired
    pub fn get_restart_required(&self) -> Option<&bool> {
        self.restart_required.as_ref()
    }

    /// Sets the value of TotalTicks
    pub fn set_total_ticks(&mut self, value: u32) {
        self.total_ticks = Some(value);
    }

    /// Gets the value of TotalTicks
    pub fn get_total_ticks(&self) -> Option<&u32> {
        self.total_ticks.as_ref()
    }

    /// Sets the value of Warnings
    pub fn set_warnings(&mut self, value: Vec<String>) {
        self.warnings = value;
    }

    /// Gets the value of Warnings
    pub fn get_warnings(&self) -> &Vec<String> {
        &self.warnings
    }
}

