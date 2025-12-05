// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MMCSSEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MMCSSEvent {
    #[serde(flatten)]
    pub base: MMCSSTrace,

/// 
    #[serde(rename = "ScheduledPID")]
    pub scheduled_pid: Option<u32>,

/// 
    #[serde(rename = "ScheduledTID")]
    pub scheduled_tid: Option<u32>,

/// 
    #[serde(rename = "SchedulingPriority")]
    pub scheduling_priority: Option<u32>,

/// 
    #[serde(rename = "TaskIndex")]
    pub task_index: Option<u32>,
}

impl MMCSSEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MMCSSTrace::new(),
            scheduled_pid: None,
            scheduled_tid: None,
            scheduling_priority: None,
            task_index: None,
        }
    }


    /// Sets the value of ScheduledPID
    pub fn set_scheduled_pid(&mut self, value: u32) {
        self.scheduled_pid = Some(value);
    }

    /// Gets the value of ScheduledPID
    pub fn get_scheduled_pid(&self) -> Option<&u32> {
        self.scheduled_pid.as_ref()
    }

    /// Sets the value of ScheduledTID
    pub fn set_scheduled_tid(&mut self, value: u32) {
        self.scheduled_tid = Some(value);
    }

    /// Gets the value of ScheduledTID
    pub fn get_scheduled_tid(&self) -> Option<&u32> {
        self.scheduled_tid.as_ref()
    }

    /// Sets the value of SchedulingPriority
    pub fn set_scheduling_priority(&mut self, value: u32) {
        self.scheduling_priority = Some(value);
    }

    /// Gets the value of SchedulingPriority
    pub fn get_scheduling_priority(&self) -> Option<&u32> {
        self.scheduling_priority.as_ref()
    }

    /// Sets the value of TaskIndex
    pub fn set_task_index(&mut self, value: u32) {
        self.task_index = Some(value);
    }

    /// Gets the value of TaskIndex
    pub fn get_task_index(&self) -> Option<&u32> {
        self.task_index.as_ref()
    }
}

