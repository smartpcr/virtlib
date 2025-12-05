// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSKeyboard_ClassInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSKeyboard_ClassInformation {
    #[serde(flatten)]
    pub base: MSKeyboard,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "DeviceId")]
    pub device_id: Option<u64>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MSKeyboard_ClassInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSKeyboard::new(),
            active: None,
            device_id: None,
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

    /// Sets the value of DeviceId
    pub fn set_device_id(&mut self, value: u64) {
        self.device_id = Some(value);
    }

    /// Gets the value of DeviceId
    pub fn get_device_id(&self) -> Option<&u64> {
        self.device_id.as_ref()
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

