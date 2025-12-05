// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFC_FibrePortNPIVAttributes struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFC_FibrePortNPIVAttributes {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "NumberVirtualPorts")]
    pub number_virtual_ports: Option<u32>,

/// 
    #[serde(rename = "VirtualPorts")]
    pub virtual_ports: Vec<MSFC_VirtualFibrePortAttributes>,

/// 
    #[serde(rename = "WWNN")]
    pub wwnn: Vec<u8>,

/// 
    #[serde(rename = "WWPN")]
    pub wwpn: Vec<u8>,
}

impl MSFC_FibrePortNPIVAttributes {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            instance_name: None,
            number_virtual_ports: None,
            virtual_ports: Vec::new(),
            wwnn: Vec::new(),
            wwpn: Vec::new(),
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

    /// Sets the value of NumberVirtualPorts
    pub fn set_number_virtual_ports(&mut self, value: u32) {
        self.number_virtual_ports = Some(value);
    }

    /// Gets the value of NumberVirtualPorts
    pub fn get_number_virtual_ports(&self) -> Option<&u32> {
        self.number_virtual_ports.as_ref()
    }

    /// Sets the value of VirtualPorts
    pub fn set_virtual_ports(&mut self, value: Vec<MSFC_VirtualFibrePortAttributes>) {
        self.virtual_ports = value;
    }

    /// Gets the value of VirtualPorts
    pub fn get_virtual_ports(&self) -> &Vec<MSFC_VirtualFibrePortAttributes> {
        &self.virtual_ports
    }

    /// Sets the value of WWNN
    pub fn set_wwnn(&mut self, value: Vec<u8>) {
        self.wwnn = value;
    }

    /// Gets the value of WWNN
    pub fn get_wwnn(&self) -> &Vec<u8> {
        &self.wwnn
    }

    /// Sets the value of WWPN
    pub fn set_wwpn(&mut self, value: Vec<u8>) {
        self.wwpn = value;
    }

    /// Gets the value of WWPN
    pub fn get_wwpn(&self) -> &Vec<u8> {
        &self.wwpn
    }
}

