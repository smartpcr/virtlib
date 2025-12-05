// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.TaskScheduler
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TaskTrigger struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TaskTrigger {

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "EndBoundary")]
    pub end_boundary: Option<String>,

/// 
    #[serde(rename = "ExecutionTimeLimit")]
    pub execution_time_limit: Option<String>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "Repetition")]
    pub repetition: Option<MSFT_TaskRepetitionPattern>,

/// 
    #[serde(rename = "StartBoundary")]
    pub start_boundary: Option<String>,
}

impl MSFT_TaskTrigger {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            enabled: None,
            end_boundary: None,
            execution_time_limit: None,
            id: None,
            repetition: None,
            start_boundary: None,
        }
    }


    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of EndBoundary
    pub fn set_end_boundary(&mut self, value: String) {
        self.end_boundary = Some(value);
    }

    /// Gets the value of EndBoundary
    pub fn get_end_boundary(&self) -> Option<&String> {
        self.end_boundary.as_ref()
    }

    /// Sets the value of ExecutionTimeLimit
    pub fn set_execution_time_limit(&mut self, value: String) {
        self.execution_time_limit = Some(value);
    }

    /// Gets the value of ExecutionTimeLimit
    pub fn get_execution_time_limit(&self) -> Option<&String> {
        self.execution_time_limit.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of Repetition
    pub fn set_repetition(&mut self, value: MSFT_TaskRepetitionPattern) {
        self.repetition = Some(value);
    }

    /// Gets the value of Repetition
    pub fn get_repetition(&self) -> Option<&MSFT_TaskRepetitionPattern> {
        self.repetition.as_ref()
    }

    /// Sets the value of StartBoundary
    pub fn set_start_boundary(&mut self, value: String) {
        self.start_boundary = Some(value);
    }

    /// Gets the value of StartBoundary
    pub fn get_start_boundary(&self) -> Option<&String> {
        self.start_boundary.as_ref()
    }
}

