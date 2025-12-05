// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSSerial_PortName struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSSerial_PortName {
    #[serde(flatten)]
    pub base: MSSerial,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "PortName")]
    pub port_name: Option<String>,
}

impl MSSerial_PortName {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSSerial::new(),
            active: None,
            instance_name: None,
            port_name: None,
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

    /// Sets the value of PortName
    pub fn set_port_name(&mut self, value: String) {
        self.port_name = Some(value);
    }

    /// Gets the value of PortName
    pub fn get_port_name(&self) -> Option<&String> {
        self.port_name.as_ref()
    }
}

