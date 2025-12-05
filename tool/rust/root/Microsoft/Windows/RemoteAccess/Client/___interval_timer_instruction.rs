// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __IntervalTimerInstruction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __IntervalTimerInstruction {
    #[serde(flatten)]
    pub base: __TimerInstruction,

/// 
    #[serde(rename = "IntervalBetweenEvents")]
    pub interval_between_events: Option<u32>,
}

impl __IntervalTimerInstruction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __TimerInstruction::new(),
            interval_between_events: None,
        }
    }


    /// Sets the value of IntervalBetweenEvents
    pub fn set_interval_between_events(&mut self, value: u32) {
        self.interval_between_events = Some(value);
    }

    /// Gets the value of IntervalBetweenEvents
    pub fn get_interval_between_events(&self) -> Option<&u32> {
        self.interval_between_events.as_ref()
    }
}

