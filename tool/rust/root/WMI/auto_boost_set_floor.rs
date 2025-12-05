// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// AutoBoostSetFloor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AutoBoostSetFloor {
    #[serde(flatten)]
    pub base: Thread_V2,

/// 
    #[serde(rename = "BoostFlags")]
    pub boost_flags: Option<u8>,

/// 
    #[serde(rename = "IoPriorities")]
    pub io_priorities: Option<u8>,

/// 
    #[serde(rename = "Lock")]
    pub lock: Option<u32>,

/// 
    #[serde(rename = "NewCpuPriorityFloor")]
    pub new_cpu_priority_floor: Option<u8>,

/// 
    #[serde(rename = "OldCpuPriority")]
    pub old_cpu_priority: Option<u8>,

/// 
    #[serde(rename = "ThreadId")]
    pub thread_id: Option<u32>,
}

impl AutoBoostSetFloor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Thread_V2::new(),
            boost_flags: None,
            io_priorities: None,
            lock: None,
            new_cpu_priority_floor: None,
            old_cpu_priority: None,
            thread_id: None,
        }
    }


    /// Sets the value of BoostFlags
    pub fn set_boost_flags(&mut self, value: u8) {
        self.boost_flags = Some(value);
    }

    /// Gets the value of BoostFlags
    pub fn get_boost_flags(&self) -> Option<&u8> {
        self.boost_flags.as_ref()
    }

    /// Sets the value of IoPriorities
    pub fn set_io_priorities(&mut self, value: u8) {
        self.io_priorities = Some(value);
    }

    /// Gets the value of IoPriorities
    pub fn get_io_priorities(&self) -> Option<&u8> {
        self.io_priorities.as_ref()
    }

    /// Sets the value of Lock
    pub fn set_lock(&mut self, value: u32) {
        self.lock = Some(value);
    }

    /// Gets the value of Lock
    pub fn get_lock(&self) -> Option<&u32> {
        self.lock.as_ref()
    }

    /// Sets the value of NewCpuPriorityFloor
    pub fn set_new_cpu_priority_floor(&mut self, value: u8) {
        self.new_cpu_priority_floor = Some(value);
    }

    /// Gets the value of NewCpuPriorityFloor
    pub fn get_new_cpu_priority_floor(&self) -> Option<&u8> {
        self.new_cpu_priority_floor.as_ref()
    }

    /// Sets the value of OldCpuPriority
    pub fn set_old_cpu_priority(&mut self, value: u8) {
        self.old_cpu_priority = Some(value);
    }

    /// Gets the value of OldCpuPriority
    pub fn get_old_cpu_priority(&self) -> Option<&u8> {
        self.old_cpu_priority.as_ref()
    }

    /// Sets the value of ThreadId
    pub fn set_thread_id(&mut self, value: u32) {
        self.thread_id = Some(value);
    }

    /// Gets the value of ThreadId
    pub fn get_thread_id(&self) -> Option<&u32> {
        self.thread_id.as_ref()
    }
}

