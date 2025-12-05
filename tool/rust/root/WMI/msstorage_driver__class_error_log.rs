// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSStorageDriver_ClassErrorLog struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSStorageDriver_ClassErrorLog {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// Error Log Array
    #[serde(rename = "logEntries")]
    pub log_entries: Vec<MSStorageDriver_ClassErrorLogEntry>,

/// Number of Error Log Entries
    #[serde(rename = "numEntries")]
    pub num_entries: Option<u32>,
}

impl MSStorageDriver_ClassErrorLog {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            instance_name: None,
            log_entries: Vec::new(),
            num_entries: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of logEntries
    pub fn set_log_entries(&mut self, value: Vec<MSStorageDriver_ClassErrorLogEntry>) {
        self.log_entries = value;
    }

    /// Gets the value of logEntries
    pub fn get_log_entries(&self) -> &Vec<MSStorageDriver_ClassErrorLogEntry> {
        &self.log_entries
    }

    /// Sets the value of numEntries
    pub fn set_num_entries(&mut self, value: u32) {
        self.num_entries = Some(value);
    }

    /// Gets the value of numEntries
    pub fn get_num_entries(&self) -> Option<&u32> {
        self.num_entries.as_ref()
    }
}

