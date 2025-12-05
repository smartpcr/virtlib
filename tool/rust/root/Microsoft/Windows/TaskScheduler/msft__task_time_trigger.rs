// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.TaskScheduler
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TaskTimeTrigger struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TaskTimeTrigger {
    #[serde(flatten)]
    pub base: MSFT_TaskTrigger,

/// 
    #[serde(rename = "RandomDelay")]
    pub random_delay: Option<String>,
}

impl MSFT_TaskTimeTrigger {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_TaskTrigger::new(),
            random_delay: None,
        }
    }


    /// Sets the value of RandomDelay
    pub fn set_random_delay(&mut self, value: String) {
        self.random_delay = Some(value);
    }

    /// Gets the value of RandomDelay
    pub fn get_random_delay(&self) -> Option<&String> {
        self.random_delay.as_ref()
    }
}

