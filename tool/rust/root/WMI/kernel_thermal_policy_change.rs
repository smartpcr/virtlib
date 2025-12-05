// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// KernelThermalPolicyChange struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KernelThermalPolicyChange {
    #[serde(flatten)]
    pub base: WMIEvent,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "CoolingMode")]
    pub cooling_mode: Option<u8>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "Processors")]
    pub processors: Option<u64>,
}

impl KernelThermalPolicyChange {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WMIEvent::new(),
            active: None,
            cooling_mode: None,
            instance_name: None,
            processors: None,
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

    /// Sets the value of CoolingMode
    pub fn set_cooling_mode(&mut self, value: u8) {
        self.cooling_mode = Some(value);
    }

    /// Gets the value of CoolingMode
    pub fn get_cooling_mode(&self) -> Option<&u8> {
        self.cooling_mode.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of Processors
    pub fn set_processors(&mut self, value: u64) {
        self.processors = Some(value);
    }

    /// Gets the value of Processors
    pub fn get_processors(&self) -> Option<&u64> {
        self.processors.as_ref()
    }
}

