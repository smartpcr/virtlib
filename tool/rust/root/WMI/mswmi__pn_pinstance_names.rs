// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSWmi_PnPInstanceNames struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSWmi_PnPInstanceNames {
    #[serde(flatten)]
    pub base: MS_WmiInternal,

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
    #[serde(rename = "InstanceNameList")]
    pub instance_name_list: Vec<String>,
}

impl MSWmi_PnPInstanceNames {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MS_WmiInternal::new(),
            active: None,
            count: None,
            instance_name: None,
            instance_name_list: Vec::new(),
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

    /// Sets the value of InstanceNameList
    pub fn set_instance_name_list(&mut self, value: Vec<String>) {
        self.instance_name_list = value;
    }

    /// Gets the value of InstanceNameList
    pub fn get_instance_name_list(&self) -> &Vec<String> {
        &self.instance_name_list
    }
}

