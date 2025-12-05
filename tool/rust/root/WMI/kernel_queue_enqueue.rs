// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// KernelQueueEnqueue struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KernelQueueEnqueue {
    #[serde(flatten)]
    pub base: Thread_V2,

/// 
    #[serde(rename = "Entry")]
    pub entry: Option<u32>,

/// 
    #[serde(rename = "ThreadId")]
    pub thread_id: Option<u32>,
}

impl KernelQueueEnqueue {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Thread_V2::new(),
            entry: None,
            thread_id: None,
        }
    }


    /// Sets the value of Entry
    pub fn set_entry(&mut self, value: u32) {
        self.entry = Some(value);
    }

    /// Gets the value of Entry
    pub fn get_entry(&self) -> Option<&u32> {
        self.entry.as_ref()
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

