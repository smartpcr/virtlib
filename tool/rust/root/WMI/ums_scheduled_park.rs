// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// UmsScheduledPark struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UmsScheduledPark {
    #[serde(flatten)]
    pub base: UmsEvent,

/// 
    #[serde(rename = "ParkFlags")]
    pub park_flags: Option<u32>,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "ScheduledThreadId")]
    pub scheduled_thread_id: Option<u32>,
}

impl UmsScheduledPark {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: UmsEvent::new(),
            park_flags: None,
            process_id: None,
            scheduled_thread_id: None,
        }
    }


    /// Sets the value of ParkFlags
    pub fn set_park_flags(&mut self, value: u32) {
        self.park_flags = Some(value);
    }

    /// Gets the value of ParkFlags
    pub fn get_park_flags(&self) -> Option<&u32> {
        self.park_flags.as_ref()
    }

    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of ScheduledThreadId
    pub fn set_scheduled_thread_id(&mut self, value: u32) {
        self.scheduled_thread_id = Some(value);
    }

    /// Gets the value of ScheduledThreadId
    pub fn get_scheduled_thread_id(&self) -> Option<&u32> {
        self.scheduled_thread_id.as_ref()
    }
}

