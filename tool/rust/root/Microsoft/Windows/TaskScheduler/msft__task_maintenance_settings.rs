// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.TaskScheduler
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TaskMaintenanceSettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TaskMaintenanceSettings {

/// 
    #[serde(rename = "Deadline")]
    pub deadline: Option<String>,

/// 
    #[serde(rename = "Exclusive")]
    pub exclusive: Option<bool>,

/// 
    #[serde(rename = "Period")]
    pub period: Option<String>,
}

impl MSFT_TaskMaintenanceSettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            deadline: None,
            exclusive: None,
            period: None,
        }
    }


    /// Sets the value of Deadline
    pub fn set_deadline(&mut self, value: String) {
        self.deadline = Some(value);
    }

    /// Gets the value of Deadline
    pub fn get_deadline(&self) -> Option<&String> {
        self.deadline.as_ref()
    }

    /// Sets the value of Exclusive
    pub fn set_exclusive(&mut self, value: bool) {
        self.exclusive = Some(value);
    }

    /// Gets the value of Exclusive
    pub fn get_exclusive(&self) -> Option<&bool> {
        self.exclusive.as_ref()
    }

    /// Sets the value of Period
    pub fn set_period(&mut self, value: String) {
        self.period = Some(value);
    }

    /// Gets the value of Period
    pub fn get_period(&self) -> Option<&String> {
        self.period.as_ref()
    }
}

