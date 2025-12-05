// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MS_SM_AdapterEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MS_SM_AdapterEvent {
    #[serde(flatten)]
    pub base: WMIEvent,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "EventType")]
    pub event_type: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "PortWWN")]
    pub port_wwn: Vec<u8>,
}

impl MS_SM_AdapterEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WMIEvent::new(),
            active: None,
            event_type: None,
            instance_name: None,
            port_wwn: Vec::new(),
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

    /// Sets the value of EventType
    pub fn set_event_type(&mut self, value: u32) {
        self.event_type = Some(value);
    }

    /// Gets the value of EventType
    pub fn get_event_type(&self) -> Option<&u32> {
        self.event_type.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of PortWWN
    pub fn set_port_wwn(&mut self, value: Vec<u8>) {
        self.port_wwn = value;
    }

    /// Gets the value of PortWWN
    pub fn get_port_wwn(&self) -> &Vec<u8> {
        &self.port_wwn
    }
}

