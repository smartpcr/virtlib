// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_StatusTaskOffloadChange struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_StatusTaskOffloadChange {
    #[serde(flatten)]
    pub base: WMIEvent,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "NumberElements")]
    pub number_elements: Option<u32>,

/// 
    #[serde(rename = "TaskOffloadCapabilities")]
    pub task_offload_capabilities: Vec<u8>,
}

impl MSNdis_StatusTaskOffloadChange {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WMIEvent::new(),
            active: None,
            instance_name: None,
            number_elements: None,
            task_offload_capabilities: Vec::new(),
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

    /// Sets the value of NumberElements
    pub fn set_number_elements(&mut self, value: u32) {
        self.number_elements = Some(value);
    }

    /// Gets the value of NumberElements
    pub fn get_number_elements(&self) -> Option<&u32> {
        self.number_elements.as_ref()
    }

    /// Sets the value of TaskOffloadCapabilities
    pub fn set_task_offload_capabilities(&mut self, value: Vec<u8>) {
        self.task_offload_capabilities = value;
    }

    /// Gets the value of TaskOffloadCapabilities
    pub fn get_task_offload_capabilities(&self) -> &Vec<u8> {
        &self.task_offload_capabilities
    }
}

