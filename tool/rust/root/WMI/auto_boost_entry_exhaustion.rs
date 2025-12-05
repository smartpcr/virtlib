// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// AutoBoostEntryExhaustion struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AutoBoostEntryExhaustion {
    #[serde(flatten)]
    pub base: Thread_V2,

/// 
    #[serde(rename = "LockAddress")]
    pub lock_address: Option<u32>,

/// 
    #[serde(rename = "ThreadId")]
    pub thread_id: Option<u32>,
}

impl AutoBoostEntryExhaustion {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Thread_V2::new(),
            lock_address: None,
            thread_id: None,
        }
    }


    /// Sets the value of LockAddress
    pub fn set_lock_address(&mut self, value: u32) {
        self.lock_address = Some(value);
    }

    /// Gets the value of LockAddress
    pub fn get_lock_address(&self) -> Option<&u32> {
        self.lock_address.as_ref()
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

