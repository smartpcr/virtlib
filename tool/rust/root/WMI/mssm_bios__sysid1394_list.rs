// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSSmBios_Sysid1394List struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSSmBios_Sysid1394List {
    #[serde(flatten)]
    pub base: MS_SmBios,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "Count")]
    pub count: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "List")]
    pub list: Vec<MSSmBios_Sysid1394>,
}

impl MSSmBios_Sysid1394List {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MS_SmBios::new(),
            active: None,
            count: None,
            instance_name: None,
            list: Vec::new(),
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

    /// Sets the value of Count
    pub fn set_count(&mut self, value: u32) {
        self.count = Some(value);
    }

    /// Gets the value of Count
    pub fn get_count(&self) -> Option<&u32> {
        self.count.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of List
    pub fn set_list(&mut self, value: Vec<MSSmBios_Sysid1394>) {
        self.list = value;
    }

    /// Gets the value of List
    pub fn get_list(&self) -> &Vec<MSSmBios_Sysid1394> {
        &self.list
    }
}

