// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Policy
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __TimerInstruction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __TimerInstruction {
    #[serde(flatten)]
    pub base: __EventGenerator,

/// 
    #[serde(rename = "SkipIfPassed")]
    pub skip_if_passed: Option<bool>,

/// 
    #[serde(rename = "TimerId")]
    pub timer_id: Option<String>,
}

impl __TimerInstruction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __EventGenerator::new(),
            skip_if_passed: None,
            timer_id: None,
        }
    }


    /// Sets the value of SkipIfPassed
    pub fn set_skip_if_passed(&mut self, value: bool) {
        self.skip_if_passed = Some(value);
    }

    /// Gets the value of SkipIfPassed
    pub fn get_skip_if_passed(&self) -> Option<&bool> {
        self.skip_if_passed.as_ref()
    }

    /// Sets the value of TimerId
    pub fn set_timer_id(&mut self, value: String) {
        self.timer_id = Some(value);
    }

    /// Gets the value of TimerId
    pub fn get_timer_id(&self) -> Option<&String> {
        self.timer_id.as_ref()
    }
}

