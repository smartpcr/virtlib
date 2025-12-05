// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __TimerEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __TimerEvent {
    #[serde(flatten)]
    pub base: __Event,

/// 
    #[serde(rename = "NumFirings")]
    pub num_firings: Option<u32>,

/// 
    #[serde(rename = "TimerId")]
    pub timer_id: Option<String>,
}

impl __TimerEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __Event::new(),
            num_firings: None,
            timer_id: None,
        }
    }


    /// Sets the value of NumFirings
    pub fn set_num_firings(&mut self, value: u32) {
        self.num_firings = Some(value);
    }

    /// Gets the value of NumFirings
    pub fn get_num_firings(&self) -> Option<&u32> {
        self.num_firings.as_ref()
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

