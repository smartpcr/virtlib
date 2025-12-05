// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// KernelQueueDequeue struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KernelQueueDequeue {
    #[serde(flatten)]
    pub base: Thread_V2,

/// 
    #[serde(rename = "Entries")]
    pub entries: Vec<u32>,

/// 
    #[serde(rename = "EntryCount")]
    pub entry_count: Option<u32>,

/// 
    #[serde(rename = "ThreadId")]
    pub thread_id: Option<u32>,
}

impl KernelQueueDequeue {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Thread_V2::new(),
            entries: Vec::new(),
            entry_count: None,
            thread_id: None,
        }
    }


    /// Sets the value of Entries
    pub fn set_entries(&mut self, value: Vec<u32>) {
        self.entries = value;
    }

    /// Gets the value of Entries
    pub fn get_entries(&self) -> &Vec<u32> {
        &self.entries
    }

    /// Sets the value of EntryCount
    pub fn set_entry_count(&mut self, value: u32) {
        self.entry_count = Some(value);
    }

    /// Gets the value of EntryCount
    pub fn get_entry_count(&self) -> Option<&u32> {
        self.entry_count.as_ref()
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

