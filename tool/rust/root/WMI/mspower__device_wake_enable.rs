// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSPower_DeviceWakeEnable struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSPower_DeviceWakeEnable {
    #[serde(flatten)]
    pub base: MSPower,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "Enable")]
    pub enable: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MSPower_DeviceWakeEnable {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSPower::new(),
            active: None,
            enable: None,
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

    /// Sets the value of Enable
    pub fn set_enable(&mut self, value: bool) {
        self.enable = Some(value);
    }

    /// Gets the value of Enable
    pub fn get_enable(&self) -> Option<&bool> {
        self.enable.as_ref()
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

