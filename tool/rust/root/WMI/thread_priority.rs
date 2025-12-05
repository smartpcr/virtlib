// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ThreadPriority struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ThreadPriority {
    #[serde(flatten)]
    pub base: Thread_V3,

/// 
    #[serde(rename = "NewPriority")]
    pub new_priority: Option<u8>,

/// 
    #[serde(rename = "OldPriority")]
    pub old_priority: Option<u8>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u16>,

/// 
    #[serde(rename = "ThreadId")]
    pub thread_id: Option<u32>,
}

impl ThreadPriority {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Thread_V3::new(),
            new_priority: None,
            old_priority: None,
            reserved: None,
            thread_id: None,
        }
    }


    /// Sets the value of NewPriority
    pub fn set_new_priority(&mut self, value: u8) {
        self.new_priority = Some(value);
    }

    /// Gets the value of NewPriority
    pub fn get_new_priority(&self) -> Option<&u8> {
        self.new_priority.as_ref()
    }

    /// Sets the value of OldPriority
    pub fn set_old_priority(&mut self, value: u8) {
        self.old_priority = Some(value);
    }

    /// Gets the value of OldPriority
    pub fn get_old_priority(&self) -> Option<&u8> {
        self.old_priority.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u16) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u16> {
        self.reserved.as_ref()
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

