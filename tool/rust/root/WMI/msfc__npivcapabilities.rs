// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFC_NPIVCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFC_NPIVCapabilities {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "DhChapAvailableOnPhysicalPort")]
    pub dh_chap_available_on_physical_port: Option<bool>,

/// 
    #[serde(rename = "DhChapAvailableOnVirtualPorts")]
    pub dh_chap_available_on_virtual_ports: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "MaxVirtualPortCount")]
    pub max_virtual_port_count: Option<u16>,
}

impl MSFC_NPIVCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            dh_chap_available_on_physical_port: None,
            dh_chap_available_on_virtual_ports: None,
            instance_name: None,
            max_virtual_port_count: None,
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

    /// Sets the value of DhChapAvailableOnPhysicalPort
    pub fn set_dh_chap_available_on_physical_port(&mut self, value: bool) {
        self.dh_chap_available_on_physical_port = Some(value);
    }

    /// Gets the value of DhChapAvailableOnPhysicalPort
    pub fn get_dh_chap_available_on_physical_port(&self) -> Option<&bool> {
        self.dh_chap_available_on_physical_port.as_ref()
    }

    /// Sets the value of DhChapAvailableOnVirtualPorts
    pub fn set_dh_chap_available_on_virtual_ports(&mut self, value: bool) {
        self.dh_chap_available_on_virtual_ports = Some(value);
    }

    /// Gets the value of DhChapAvailableOnVirtualPorts
    pub fn get_dh_chap_available_on_virtual_ports(&self) -> Option<&bool> {
        self.dh_chap_available_on_virtual_ports.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of MaxVirtualPortCount
    pub fn set_max_virtual_port_count(&mut self, value: u16) {
        self.max_virtual_port_count = Some(value);
    }

    /// Gets the value of MaxVirtualPortCount
    pub fn get_max_virtual_port_count(&self) -> Option<&u16> {
        self.max_virtual_port_count.as_ref()
    }
}

