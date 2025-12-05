// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ThreadSetName struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ThreadSetName {
    #[serde(flatten)]
    pub base: Thread_V2,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "ThreadId")]
    pub thread_id: Option<u32>,

/// 
    #[serde(rename = "ThreadName")]
    pub thread_name: Option<String>,
}

impl ThreadSetName {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Thread_V2::new(),
            process_id: None,
            thread_id: None,
            thread_name: None,
        }
    }


    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of ThreadId
    pub fn set_thread_id(&mut self, value: u32) {
        self.thread_id = Some(value);
    }

    /// Gets the value of ThreadId
    pub fn get_thread_id(&self) -> Option<&u32> {
        self.thread_id.as_ref()
    }

    /// Sets the value of ThreadName
    pub fn set_thread_name(&mut self, value: String) {
        self.thread_name = Some(value);
    }

    /// Gets the value of ThreadName
    pub fn get_thread_name(&self) -> Option<&String> {
        self.thread_name.as_ref()
    }
}

