// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ReadyThread struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReadyThread {
    #[serde(flatten)]
    pub base: Thread_V2,

/// 
    #[serde(rename = "AdjustIncrement")]
    pub adjust_increment: Option<u8>,

/// 
    #[serde(rename = "AdjustReason")]
    pub adjust_reason: Option<u8>,

/// 
    #[serde(rename = "Flag")]
    pub flag: Option<u8>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u8>,

/// 
    #[serde(rename = "TThreadId")]
    pub tthread_id: Option<u32>,
}

impl ReadyThread {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Thread_V2::new(),
            adjust_increment: None,
            adjust_reason: None,
            flag: None,
            reserved: None,
            tthread_id: None,
        }
    }


    /// Sets the value of AdjustIncrement
    pub fn set_adjust_increment(&mut self, value: u8) {
        self.adjust_increment = Some(value);
    }

    /// Gets the value of AdjustIncrement
    pub fn get_adjust_increment(&self) -> Option<&u8> {
        self.adjust_increment.as_ref()
    }

    /// Sets the value of AdjustReason
    pub fn set_adjust_reason(&mut self, value: u8) {
        self.adjust_reason = Some(value);
    }

    /// Gets the value of AdjustReason
    pub fn get_adjust_reason(&self) -> Option<&u8> {
        self.adjust_reason.as_ref()
    }

    /// Sets the value of Flag
    pub fn set_flag(&mut self, value: u8) {
        self.flag = Some(value);
    }

    /// Gets the value of Flag
    pub fn get_flag(&self) -> Option<&u8> {
        self.flag.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u8) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u8> {
        self.reserved.as_ref()
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

