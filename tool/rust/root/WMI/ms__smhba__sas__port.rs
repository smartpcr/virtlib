// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MS_SMHBA_SAS_Port struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MS_SMHBA_SAS_Port {

/// 
    #[serde(rename = "AttachedSASAddress")]
    pub attached_sasaddress: Vec<u8>,

/// 
    #[serde(rename = "LocalSASAddress")]
    pub local_sasaddress: Vec<u8>,

/// 
    #[serde(rename = "NumberofDiscoveredPorts")]
    pub numberof_discovered_ports: Option<u32>,

/// 
    #[serde(rename = "NumberofPhys")]
    pub numberof_phys: Option<u32>,

/// 
    #[serde(rename = "PortProtocol")]
    pub port_protocol: Option<u32>,
}

impl MS_SMHBA_SAS_Port {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            attached_sasaddress: Vec::new(),
            local_sasaddress: Vec::new(),
            numberof_discovered_ports: None,
            numberof_phys: None,
            port_protocol: None,
        }
    }


    /// Sets the value of AttachedSASAddress
    pub fn set_attached_sasaddress(&mut self, value: Vec<u8>) {
        self.attached_sasaddress = value;
    }

    /// Gets the value of AttachedSASAddress
    pub fn get_attached_sasaddress(&self) -> &Vec<u8> {
        &self.attached_sasaddress
    }

    /// Sets the value of LocalSASAddress
    pub fn set_local_sasaddress(&mut self, value: Vec<u8>) {
        self.local_sasaddress = value;
    }

    /// Gets the value of LocalSASAddress
    pub fn get_local_sasaddress(&self) -> &Vec<u8> {
        &self.local_sasaddress
    }

    /// Sets the value of NumberofDiscoveredPorts
    pub fn set_numberof_discovered_ports(&mut self, value: u32) {
        self.numberof_discovered_ports = Some(value);
    }

    /// Gets the value of NumberofDiscoveredPorts
    pub fn get_numberof_discovered_ports(&self) -> Option<&u32> {
        self.numberof_discovered_ports.as_ref()
    }

    /// Sets the value of NumberofPhys
    pub fn set_numberof_phys(&mut self, value: u32) {
        self.numberof_phys = Some(value);
    }

    /// Gets the value of NumberofPhys
    pub fn get_numberof_phys(&self) -> Option<&u32> {
        self.numberof_phys.as_ref()
    }

    /// Sets the value of PortProtocol
    pub fn set_port_protocol(&mut self, value: u32) {
        self.port_protocol = Some(value);
    }

    /// Gets the value of PortProtocol
    pub fn get_port_protocol(&self) -> Option<&u32> {
        self.port_protocol.as_ref()
    }
}

