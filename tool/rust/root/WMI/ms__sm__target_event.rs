// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MS_SM_TargetEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MS_SM_TargetEvent {
    #[serde(flatten)]
    pub base: WMIEvent,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "DiscoveredPortWWN")]
    pub discovered_port_wwn: Vec<u8>,

/// 
    #[serde(rename = "DomainPortWWN")]
    pub domain_port_wwn: Vec<u8>,

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

impl MS_SM_TargetEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WMIEvent::new(),
            active: None,
            discovered_port_wwn: Vec::new(),
            domain_port_wwn: Vec::new(),
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

    /// Sets the value of DiscoveredPortWWN
    pub fn set_discovered_port_wwn(&mut self, value: Vec<u8>) {
        self.discovered_port_wwn = value;
    }

    /// Gets the value of DiscoveredPortWWN
    pub fn get_discovered_port_wwn(&self) -> &Vec<u8> {
        &self.discovered_port_wwn
    }

    /// Sets the value of DomainPortWWN
    pub fn set_domain_port_wwn(&mut self, value: Vec<u8>) {
        self.domain_port_wwn = value;
    }

    /// Gets the value of DomainPortWWN
    pub fn get_domain_port_wwn(&self) -> &Vec<u8> {
        &self.domain_port_wwn
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

