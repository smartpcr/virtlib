// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSVerifierIrpLogInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSVerifierIrpLogInformation {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// DeviceType
    #[serde(rename = "DeviceType")]
    pub device_type: Option<u32>,

/// 
    #[serde(rename = "Entries")]
    pub entries: Vec<MSVerifierIrpLogEntry>,

/// 
    #[serde(rename = "EntryCount")]
    pub entry_count: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MSVerifierIrpLogInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            device_type: None,
            entries: Vec::new(),
            entry_count: None,
            instance_name: None,
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

    /// Sets the value of DeviceType
    pub fn set_device_type(&mut self, value: u32) {
        self.device_type = Some(value);
    }

    /// Gets the value of DeviceType
    pub fn get_device_type(&self) -> Option<&u32> {
        self.device_type.as_ref()
    }

    /// Sets the value of Entries
    pub fn set_entries(&mut self, value: Vec<MSVerifierIrpLogEntry>) {
        self.entries = value;
    }

    /// Gets the value of Entries
    pub fn get_entries(&self) -> &Vec<MSVerifierIrpLogEntry> {
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

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }
}

