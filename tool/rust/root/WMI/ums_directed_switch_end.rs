// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// UmsDirectedSwitchEnd struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UmsDirectedSwitchEnd {
    #[serde(flatten)]
    pub base: UmsEvent,

/// 
    #[serde(rename = "PrimaryThreadId")]
    pub primary_thread_id: Option<u32>,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "ScheduledThreadId")]
    pub scheduled_thread_id: Option<u32>,

/// 
    #[serde(rename = "SwitchFlags")]
    pub switch_flags: Option<u32>,
}

impl UmsDirectedSwitchEnd {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: UmsEvent::new(),
            primary_thread_id: None,
            process_id: None,
            scheduled_thread_id: None,
            switch_flags: None,
        }
    }


    /// Sets the value of PrimaryThreadId
    pub fn set_primary_thread_id(&mut self, value: u32) {
        self.primary_thread_id = Some(value);
    }

    /// Gets the value of PrimaryThreadId
    pub fn get_primary_thread_id(&self) -> Option<&u32> {
        self.primary_thread_id.as_ref()
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

    /// Sets the value of SwitchFlags
    pub fn set_switch_flags(&mut self, value: u32) {
        self.switch_flags = Some(value);
    }

    /// Gets the value of SwitchFlags
    pub fn get_switch_flags(&self) -> Option<&u32> {
        self.switch_flags.as_ref()
    }
}

