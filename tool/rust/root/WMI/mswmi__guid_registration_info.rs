// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSWmi_GuidRegistrationInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSWmi_GuidRegistrationInfo {
    #[serde(flatten)]
    pub base: WMIEvent,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "GuidCount")]
    pub guid_count: Option<u32>,

/// 
    #[serde(rename = "GuidList")]
    pub guid_list: Vec<MSWmi_Guid>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "Operation")]
    pub operation: Option<u32>,
}

impl MSWmi_GuidRegistrationInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WMIEvent::new(),
            active: None,
            guid_count: None,
            guid_list: Vec::new(),
            instance_name: None,
            operation: None,
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

    /// Sets the value of GuidCount
    pub fn set_guid_count(&mut self, value: u32) {
        self.guid_count = Some(value);
    }

    /// Gets the value of GuidCount
    pub fn get_guid_count(&self) -> Option<&u32> {
        self.guid_count.as_ref()
    }

    /// Sets the value of GuidList
    pub fn set_guid_list(&mut self, value: Vec<MSWmi_Guid>) {
        self.guid_list = value;
    }

    /// Gets the value of GuidList
    pub fn get_guid_list(&self) -> &Vec<MSWmi_Guid> {
        &self.guid_list
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of Operation
    pub fn set_operation(&mut self, value: u32) {
        self.operation = Some(value);
    }

    /// Gets the value of Operation
    pub fn get_operation(&self) -> Option<&u32> {
        self.operation.as_ref()
    }
}

