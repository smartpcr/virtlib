// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.TaskScheduler
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TaskRepetitionPattern struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TaskRepetitionPattern {

/// 
    #[serde(rename = "Duration")]
    pub duration: Option<String>,

/// 
    #[serde(rename = "Interval")]
    pub interval: Option<String>,

/// 
    #[serde(rename = "StopAtDurationEnd")]
    pub stop_at_duration_end: Option<bool>,
}

impl MSFT_TaskRepetitionPattern {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            duration: None,
            interval: None,
            stop_at_duration_end: None,
        }
    }


    /// Sets the value of Duration
    pub fn set_duration(&mut self, value: String) {
        self.duration = Some(value);
    }

    /// Gets the value of Duration
    pub fn get_duration(&self) -> Option<&String> {
        self.duration.as_ref()
    }

    /// Sets the value of Interval
    pub fn set_interval(&mut self, value: String) {
        self.interval = Some(value);
    }

    /// Gets the value of Interval
    pub fn get_interval(&self) -> Option<&String> {
        self.interval.as_ref()
    }

    /// Sets the value of StopAtDurationEnd
    pub fn set_stop_at_duration_end(&mut self, value: bool) {
        self.stop_at_duration_end = Some(value);
    }

    /// Gets the value of StopAtDurationEnd
    pub fn get_stop_at_duration_end(&self) -> Option<&bool> {
        self.stop_at_duration_end.as_ref()
    }
}

