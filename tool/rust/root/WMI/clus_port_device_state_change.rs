// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ClusPortDeviceStateChange struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusPortDeviceStateChange {
    #[serde(flatten)]
    pub base: WMIEvent,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// Device Guid .
    #[serde(rename = "DeviceGuid")]
    pub device_guid: Option<String>,

/// Device Number.
    #[serde(rename = "DeviceNumber")]
    pub device_number: Option<u32>,

/// ClusPort Device State.
    #[serde(rename = "DeviceState")]
    pub device_state: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl ClusPortDeviceStateChange {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WMIEvent::new(),
            active: None,
            device_guid: None,
            device_number: None,
            device_state: None,
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

    /// Sets the value of DeviceGuid
    pub fn set_device_guid(&mut self, value: String) {
        self.device_guid = Some(value);
    }

    /// Gets the value of DeviceGuid
    pub fn get_device_guid(&self) -> Option<&String> {
        self.device_guid.as_ref()
    }

    /// Sets the value of DeviceNumber
    pub fn set_device_number(&mut self, value: u32) {
        self.device_number = Some(value);
    }

    /// Gets the value of DeviceNumber
    pub fn get_device_number(&self) -> Option<&u32> {
        self.device_number.as_ref()
    }

    /// Sets the value of DeviceState
    pub fn set_device_state(&mut self, value: u32) {
        self.device_state = Some(value);
    }

    /// Gets the value of DeviceState
    pub fn get_device_state(&self) -> Option<&u32> {
        self.device_state.as_ref()
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

