// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// IoTimerEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IoTimerEvent {
    #[serde(flatten)]
    pub base: PerfInfo_V2,

/// 
    #[serde(rename = "DeviceObject")]
    pub device_object: Option<u32>,

/// 
    #[serde(rename = "TimerRoutine")]
    pub timer_routine: Option<u32>,
}

impl IoTimerEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PerfInfo_V2::new(),
            device_object: None,
            timer_routine: None,
        }
    }


    /// Sets the value of DeviceObject
    pub fn set_device_object(&mut self, value: u32) {
        self.device_object = Some(value);
    }

    /// Gets the value of DeviceObject
    pub fn get_device_object(&self) -> Option<&u32> {
        self.device_object.as_ref()
    }

    /// Sets the value of TimerRoutine
    pub fn set_timer_routine(&mut self, value: u32) {
        self.timer_routine = Some(value);
    }

    /// Gets the value of TimerRoutine
    pub fn get_timer_routine(&self) -> Option<&u32> {
        self.timer_routine.as_ref()
    }
}

