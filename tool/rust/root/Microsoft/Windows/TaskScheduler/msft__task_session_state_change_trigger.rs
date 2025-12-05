// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.TaskScheduler
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TaskSessionStateChangeTrigger struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TaskSessionStateChangeTrigger {
    #[serde(flatten)]
    pub base: MSFT_TaskTrigger,

/// 
    #[serde(rename = "Delay")]
    pub delay: Option<String>,

/// 
    #[serde(rename = "StateChange")]
    pub state_change: Option<u32>,

/// 
    #[serde(rename = "UserId")]
    pub user_id: Option<String>,
}

impl MSFT_TaskSessionStateChangeTrigger {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_TaskTrigger::new(),
            delay: None,
            state_change: None,
            user_id: None,
        }
    }


    /// Sets the value of Delay
    pub fn set_delay(&mut self, value: String) {
        self.delay = Some(value);
    }

    /// Gets the value of Delay
    pub fn get_delay(&self) -> Option<&String> {
        self.delay.as_ref()
    }

    /// Sets the value of StateChange
    pub fn set_state_change(&mut self, value: u32) {
        self.state_change = Some(value);
    }

    /// Gets the value of StateChange
    pub fn get_state_change(&self) -> Option<&u32> {
        self.state_change.as_ref()
    }

    /// Sets the value of UserId
    pub fn set_user_id(&mut self, value: String) {
        self.user_id = Some(value);
    }

    /// Gets the value of UserId
    pub fn get_user_id(&self) -> Option<&String> {
        self.user_id.as_ref()
    }
}

