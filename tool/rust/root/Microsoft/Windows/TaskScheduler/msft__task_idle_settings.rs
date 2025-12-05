// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.TaskScheduler
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TaskIdleSettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TaskIdleSettings {

/// 
    #[serde(rename = "IdleDuration")]
    pub idle_duration: Option<String>,

/// 
    #[serde(rename = "RestartOnIdle")]
    pub restart_on_idle: Option<bool>,

/// 
    #[serde(rename = "StopOnIdleEnd")]
    pub stop_on_idle_end: Option<bool>,

/// 
    #[serde(rename = "WaitTimeout")]
    pub wait_timeout: Option<String>,
}

impl MSFT_TaskIdleSettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            idle_duration: None,
            restart_on_idle: None,
            stop_on_idle_end: None,
            wait_timeout: None,
        }
    }


    /// Sets the value of IdleDuration
    pub fn set_idle_duration(&mut self, value: String) {
        self.idle_duration = Some(value);
    }

    /// Gets the value of IdleDuration
    pub fn get_idle_duration(&self) -> Option<&String> {
        self.idle_duration.as_ref()
    }

    /// Sets the value of RestartOnIdle
    pub fn set_restart_on_idle(&mut self, value: bool) {
        self.restart_on_idle = Some(value);
    }

    /// Gets the value of RestartOnIdle
    pub fn get_restart_on_idle(&self) -> Option<&bool> {
        self.restart_on_idle.as_ref()
    }

    /// Sets the value of StopOnIdleEnd
    pub fn set_stop_on_idle_end(&mut self, value: bool) {
        self.stop_on_idle_end = Some(value);
    }

    /// Gets the value of StopOnIdleEnd
    pub fn get_stop_on_idle_end(&self) -> Option<&bool> {
        self.stop_on_idle_end.as_ref()
    }

    /// Sets the value of WaitTimeout
    pub fn set_wait_timeout(&mut self, value: String) {
        self.wait_timeout = Some(value);
    }

    /// Gets the value of WaitTimeout
    pub fn get_wait_timeout(&self) -> Option<&String> {
        self.wait_timeout.as_ref()
    }
}

