// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// KernelPerfStateChange struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KernelPerfStateChange {
    #[serde(flatten)]
    pub base: WMIEvent,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "Latency")]
    pub latency: Option<u32>,

/// 
    #[serde(rename = "Processor")]
    pub processor: Option<u32>,

/// 
    #[serde(rename = "Speed")]
    pub speed: Option<u32>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u32>,
}

impl KernelPerfStateChange {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WMIEvent::new(),
            active: None,
            instance_name: None,
            latency: None,
            processor: None,
            speed: None,
            state: None,
            status: None,
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

    /// Sets the value of Latency
    pub fn set_latency(&mut self, value: u32) {
        self.latency = Some(value);
    }

    /// Gets the value of Latency
    pub fn get_latency(&self) -> Option<&u32> {
        self.latency.as_ref()
    }

    /// Sets the value of Processor
    pub fn set_processor(&mut self, value: u32) {
        self.processor = Some(value);
    }

    /// Gets the value of Processor
    pub fn get_processor(&self) -> Option<&u32> {
        self.processor.as_ref()
    }

    /// Sets the value of Speed
    pub fn set_speed(&mut self, value: u32) {
        self.speed = Some(value);
    }

    /// Gets the value of Speed
    pub fn get_speed(&self) -> Option<&u32> {
        self.speed.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u32> {
        self.state.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u32> {
        self.status.as_ref()
    }
}

