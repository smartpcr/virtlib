// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFC_LinkEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFC_LinkEvent {
    #[serde(flatten)]
    pub base: WMIEvent,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "AdapterWWN")]
    pub adapter_wwn: Vec<u8>,

/// 
    #[serde(rename = "EventType")]
    pub event_type: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "RLIRBuffer")]
    pub rlirbuffer: Vec<u8>,

/// 
    #[serde(rename = "RLIRBufferSize")]
    pub rlirbuffer_size: Option<u32>,
}

impl MSFC_LinkEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WMIEvent::new(),
            active: None,
            adapter_wwn: Vec::new(),
            event_type: None,
            instance_name: None,
            rlirbuffer: Vec::new(),
            rlirbuffer_size: None,
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

    /// Sets the value of AdapterWWN
    pub fn set_adapter_wwn(&mut self, value: Vec<u8>) {
        self.adapter_wwn = value;
    }

    /// Gets the value of AdapterWWN
    pub fn get_adapter_wwn(&self) -> &Vec<u8> {
        &self.adapter_wwn
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

    /// Sets the value of RLIRBuffer
    pub fn set_rlirbuffer(&mut self, value: Vec<u8>) {
        self.rlirbuffer = value;
    }

    /// Gets the value of RLIRBuffer
    pub fn get_rlirbuffer(&self) -> &Vec<u8> {
        &self.rlirbuffer
    }

    /// Sets the value of RLIRBufferSize
    pub fn set_rlirbuffer_size(&mut self, value: u32) {
        self.rlirbuffer_size = Some(value);
    }

    /// Gets the value of RLIRBufferSize
    pub fn get_rlirbuffer_size(&self) -> Option<&u32> {
        self.rlirbuffer_size.as_ref()
    }
}

