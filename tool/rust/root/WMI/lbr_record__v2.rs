// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// LbrRecord_V2 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LbrRecord_V2 {
    #[serde(flatten)]
    pub base: LBR_V2,

/// 
    #[serde(rename = "Entries")]
    pub entries: Vec<LbrRecordEntry>,

/// 
    #[serde(rename = "Options")]
    pub options: Option<u32>,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "ThreadId")]
    pub thread_id: Option<u32>,

/// 
    #[serde(rename = "TimeStamp")]
    pub time_stamp: Option<u64>,
}

impl LbrRecord_V2 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: LBR_V2::new(),
            entries: Vec::new(),
            options: None,
            process_id: None,
            thread_id: None,
            time_stamp: None,
        }
    }


    /// Sets the value of Entries
    pub fn set_entries(&mut self, value: Vec<LbrRecordEntry>) {
        self.entries = value;
    }

    /// Gets the value of Entries
    pub fn get_entries(&self) -> &Vec<LbrRecordEntry> {
        &self.entries
    }

    /// Sets the value of Options
    pub fn set_options(&mut self, value: u32) {
        self.options = Some(value);
    }

    /// Gets the value of Options
    pub fn get_options(&self) -> Option<&u32> {
        self.options.as_ref()
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

    /// Sets the value of TimeStamp
    pub fn set_time_stamp(&mut self, value: u64) {
        self.time_stamp = Some(value);
    }

    /// Gets the value of TimeStamp
    pub fn get_time_stamp(&self) -> Option<&u64> {
        self.time_stamp.as_ref()
    }
}

