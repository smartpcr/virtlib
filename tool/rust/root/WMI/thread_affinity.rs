// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ThreadAffinity struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ThreadAffinity {
    #[serde(flatten)]
    pub base: Thread_V2,

/// 
    #[serde(rename = "Affinity")]
    pub affinity: Option<u32>,

/// 
    #[serde(rename = "Group")]
    pub group: Option<u16>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u16>,

/// 
    #[serde(rename = "ThreadId")]
    pub thread_id: Option<u32>,
}

impl ThreadAffinity {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Thread_V2::new(),
            affinity: None,
            group: None,
            reserved: None,
            thread_id: None,
        }
    }


    /// Sets the value of Affinity
    pub fn set_affinity(&mut self, value: u32) {
        self.affinity = Some(value);
    }

    /// Gets the value of Affinity
    pub fn get_affinity(&self) -> Option<&u32> {
        self.affinity.as_ref()
    }

    /// Sets the value of Group
    pub fn set_group(&mut self, value: u16) {
        self.group = Some(value);
    }

    /// Gets the value of Group
    pub fn get_group(&self) -> Option<&u16> {
        self.group.as_ref()
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

