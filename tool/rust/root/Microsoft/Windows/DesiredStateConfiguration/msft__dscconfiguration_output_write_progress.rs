// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DSCConfigurationOutputWriteProgress struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DSCConfigurationOutputWriteProgress {
    #[serde(flatten)]
    pub base: MSFT_DSCConfigurationOutput,

/// 
    #[serde(rename = "Activity")]
    pub activity: Option<String>,

/// 
    #[serde(rename = "CurrentOperation")]
    pub current_operation: Option<String>,

/// 
    #[serde(rename = "PercentComplete")]
    pub percent_complete: Option<u32>,

/// 
    #[serde(rename = "SecondsRemaining")]
    pub seconds_remaining: Option<u32>,

/// 
    #[serde(rename = "StatusDescription")]
    pub status_description: Option<String>,
}

impl MSFT_DSCConfigurationOutputWriteProgress {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_DSCConfigurationOutput::new(),
            activity: None,
            current_operation: None,
            percent_complete: None,
            seconds_remaining: None,
            status_description: None,
        }
    }


    /// Sets the value of Activity
    pub fn set_activity(&mut self, value: String) {
        self.activity = Some(value);
    }

    /// Gets the value of Activity
    pub fn get_activity(&self) -> Option<&String> {
        self.activity.as_ref()
    }

    /// Sets the value of CurrentOperation
    pub fn set_current_operation(&mut self, value: String) {
        self.current_operation = Some(value);
    }

    /// Gets the value of CurrentOperation
    pub fn get_current_operation(&self) -> Option<&String> {
        self.current_operation.as_ref()
    }

    /// Sets the value of PercentComplete
    pub fn set_percent_complete(&mut self, value: u32) {
        self.percent_complete = Some(value);
    }

    /// Gets the value of PercentComplete
    pub fn get_percent_complete(&self) -> Option<&u32> {
        self.percent_complete.as_ref()
    }

    /// Sets the value of SecondsRemaining
    pub fn set_seconds_remaining(&mut self, value: u32) {
        self.seconds_remaining = Some(value);
    }

    /// Gets the value of SecondsRemaining
    pub fn get_seconds_remaining(&self) -> Option<&u32> {
        self.seconds_remaining.as_ref()
    }

    /// Sets the value of StatusDescription
    pub fn set_status_description(&mut self, value: String) {
        self.status_description = Some(value);
    }

    /// Gets the value of StatusDescription
    pub fn get_status_description(&self) -> Option<&String> {
        self.status_description.as_ref()
    }
}

