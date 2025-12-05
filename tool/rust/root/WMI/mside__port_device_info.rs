// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSIde_PortDeviceInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSIde_PortDeviceInfo {
    #[serde(flatten)]
    pub base: MSIde,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "Bus")]
    pub bus: Option<u8>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "Lun")]
    pub lun: Option<u8>,

/// 
    #[serde(rename = "Target")]
    pub target: Option<u8>,
}

impl MSIde_PortDeviceInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSIde::new(),
            active: None,
            bus: None,
            instance_name: None,
            lun: None,
            target: None,
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

    /// Sets the value of Bus
    pub fn set_bus(&mut self, value: u8) {
        self.bus = Some(value);
    }

    /// Gets the value of Bus
    pub fn get_bus(&self) -> Option<&u8> {
        self.bus.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of Lun
    pub fn set_lun(&mut self, value: u8) {
        self.lun = Some(value);
    }

    /// Gets the value of Lun
    pub fn get_lun(&self) -> Option<&u8> {
        self.lun.as_ref()
    }

    /// Sets the value of Target
    pub fn set_target(&mut self, value: u8) {
        self.target = Some(value);
    }

    /// Gets the value of Target
    pub fn get_target(&self) -> Option<&u8> {
        self.target.as_ref()
    }
}

