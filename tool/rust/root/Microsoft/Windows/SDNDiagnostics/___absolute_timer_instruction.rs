// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SDNDiagnostics
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __AbsoluteTimerInstruction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __AbsoluteTimerInstruction {
    #[serde(flatten)]
    pub base: __TimerInstruction,

/// 
    #[serde(rename = "EventDateTime")]
    pub event_date_time: Option<String>,
}

impl __AbsoluteTimerInstruction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __TimerInstruction::new(),
            event_date_time: None,
        }
    }


    /// Sets the value of EventDateTime
    pub fn set_event_date_time(&mut self, value: String) {
        self.event_date_time = Some(value);
    }

    /// Gets the value of EventDateTime
    pub fn get_event_date_time(&self) -> Option<&String> {
        self.event_date_time.as_ref()
    }
}

