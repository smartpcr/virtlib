// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WorkerThread struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkerThread {
    #[serde(flatten)]
    pub base: Thread_V2,

/// 
    #[serde(rename = "StartTime")]
    pub start_time: Option<u64>,

/// 
    #[serde(rename = "ThreadRoutine")]
    pub thread_routine: Option<u32>,

/// 
    #[serde(rename = "TThreadId")]
    pub tthread_id: Option<u32>,
}

impl WorkerThread {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Thread_V2::new(),
            start_time: None,
            thread_routine: None,
            tthread_id: None,
        }
    }


    /// Sets the value of StartTime
    pub fn set_start_time(&mut self, value: u64) {
        self.start_time = Some(value);
    }

    /// Gets the value of StartTime
    pub fn get_start_time(&self) -> Option<&u64> {
        self.start_time.as_ref()
    }

    /// Sets the value of ThreadRoutine
    pub fn set_thread_routine(&mut self, value: u32) {
        self.thread_routine = Some(value);
    }

    /// Gets the value of ThreadRoutine
    pub fn get_thread_routine(&self) -> Option<&u32> {
        self.thread_routine.as_ref()
    }

    /// Sets the value of TThreadId
    pub fn set_tthread_id(&mut self, value: u32) {
        self.tthread_id = Some(value);
    }

    /// Gets the value of TThreadId
    pub fn get_tthread_id(&self) -> Option<&u32> {
        self.tthread_id.as_ref()
    }
}

