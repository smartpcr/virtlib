// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CancelKTimer2 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CancelKTimer2 {
    #[serde(flatten)]
    pub base: PerfInfo_V2,

/// 
    #[serde(rename = "Timer")]
    pub timer: Option<u32>,
}

impl CancelKTimer2 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PerfInfo_V2::new(),
            timer: None,
        }
    }


    /// Sets the value of Timer
    pub fn set_timer(&mut self, value: u32) {
        self.timer = Some(value);
    }

    /// Gets the value of Timer
    pub fn get_timer(&self) -> Option<&u32> {
        self.timer.as_ref()
    }
}

