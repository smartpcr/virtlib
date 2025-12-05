// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_StatusDot11ConnectionStart struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_StatusDot11ConnectionStart {
    #[serde(flatten)]
    pub base: WMIEvent,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "NdisStatusDot11ConnectionStartIndication")]
    pub ndis_status_dot11_connection_start_indication: Vec<u8>,

/// 
    #[serde(rename = "NumberElements")]
    pub number_elements: Option<u32>,
}

impl MSNdis_StatusDot11ConnectionStart {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WMIEvent::new(),
            active: None,
            instance_name: None,
            ndis_status_dot11_connection_start_indication: Vec::new(),
            number_elements: None,
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

    /// Sets the value of NdisStatusDot11ConnectionStartIndication
    pub fn set_ndis_status_dot11_connection_start_indication(&mut self, value: Vec<u8>) {
        self.ndis_status_dot11_connection_start_indication = value;
    }

    /// Gets the value of NdisStatusDot11ConnectionStartIndication
    pub fn get_ndis_status_dot11_connection_start_indication(&self) -> &Vec<u8> {
        &self.ndis_status_dot11_connection_start_indication
    }

    /// Sets the value of NumberElements
    pub fn set_number_elements(&mut self, value: u32) {
        self.number_elements = Some(value);
    }

    /// Gets the value of NumberElements
    pub fn get_number_elements(&self) -> Option<&u32> {
        self.number_elements.as_ref()
    }
}

