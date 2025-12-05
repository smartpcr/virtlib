// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WmiMonitorBrightness struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WmiMonitorBrightness {
    #[serde(flatten)]
    pub base: MSMonitorClass,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "CurrentBrightness")]
    pub current_brightness: Option<u8>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "Level")]
    pub level: Vec<u8>,

/// 
    #[serde(rename = "Levels")]
    pub levels: Option<u32>,
}

impl WmiMonitorBrightness {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSMonitorClass::new(),
            active: None,
            current_brightness: None,
            instance_name: None,
            level: Vec::new(),
            levels: None,
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

    /// Sets the value of CurrentBrightness
    pub fn set_current_brightness(&mut self, value: u8) {
        self.current_brightness = Some(value);
    }

    /// Gets the value of CurrentBrightness
    pub fn get_current_brightness(&self) -> Option<&u8> {
        self.current_brightness.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of Level
    pub fn set_level(&mut self, value: Vec<u8>) {
        self.level = value;
    }

    /// Gets the value of Level
    pub fn get_level(&self) -> &Vec<u8> {
        &self.level
    }

    /// Sets the value of Levels
    pub fn set_levels(&mut self, value: u32) {
        self.levels = Some(value);
    }

    /// Gets the value of Levels
    pub fn get_levels(&self) -> Option<&u32> {
        self.levels.as_ref()
    }
}

