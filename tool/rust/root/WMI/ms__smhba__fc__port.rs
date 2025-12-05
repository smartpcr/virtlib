// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MS_SMHBA_FC_Port struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MS_SMHBA_FC_Port {

/// 
    #[serde(rename = "FabricName")]
    pub fabric_name: Vec<u8>,

/// 
    #[serde(rename = "FcId")]
    pub fc_id: Option<u32>,

/// 
    #[serde(rename = "NodeWWN")]
    pub node_wwn: Vec<u8>,

/// 
    #[serde(rename = "NumberofDiscoveredPorts")]
    pub numberof_discovered_ports: Option<u32>,

/// 
    #[serde(rename = "NumberofPhys")]
    pub numberof_phys: Option<u8>,

/// 
    #[serde(rename = "PortActiveFc4Types")]
    pub port_active_fc4_types: Vec<u8>,

/// 
    #[serde(rename = "PortSupportedClassofService")]
    pub port_supported_classof_service: Option<u32>,

/// 
    #[serde(rename = "PortSupportedFc4Types")]
    pub port_supported_fc4_types: Vec<u8>,

/// 
    #[serde(rename = "PortSymbolicName")]
    pub port_symbolic_name: Option<String>,

/// 
    #[serde(rename = "PortWWN")]
    pub port_wwn: Vec<u8>,
}

impl MS_SMHBA_FC_Port {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            fabric_name: Vec::new(),
            fc_id: None,
            node_wwn: Vec::new(),
            numberof_discovered_ports: None,
            numberof_phys: None,
            port_active_fc4_types: Vec::new(),
            port_supported_classof_service: None,
            port_supported_fc4_types: Vec::new(),
            port_symbolic_name: None,
            port_wwn: Vec::new(),
        }
    }


    /// Sets the value of FabricName
    pub fn set_fabric_name(&mut self, value: Vec<u8>) {
        self.fabric_name = value;
    }

    /// Gets the value of FabricName
    pub fn get_fabric_name(&self) -> &Vec<u8> {
        &self.fabric_name
    }

    /// Sets the value of FcId
    pub fn set_fc_id(&mut self, value: u32) {
        self.fc_id = Some(value);
    }

    /// Gets the value of FcId
    pub fn get_fc_id(&self) -> Option<&u32> {
        self.fc_id.as_ref()
    }

    /// Sets the value of NodeWWN
    pub fn set_node_wwn(&mut self, value: Vec<u8>) {
        self.node_wwn = value;
    }

    /// Gets the value of NodeWWN
    pub fn get_node_wwn(&self) -> &Vec<u8> {
        &self.node_wwn
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
    pub fn set_numberof_phys(&mut self, value: u8) {
        self.numberof_phys = Some(value);
    }

    /// Gets the value of NumberofPhys
    pub fn get_numberof_phys(&self) -> Option<&u8> {
        self.numberof_phys.as_ref()
    }

    /// Sets the value of PortActiveFc4Types
    pub fn set_port_active_fc4_types(&mut self, value: Vec<u8>) {
        self.port_active_fc4_types = value;
    }

    /// Gets the value of PortActiveFc4Types
    pub fn get_port_active_fc4_types(&self) -> &Vec<u8> {
        &self.port_active_fc4_types
    }

    /// Sets the value of PortSupportedClassofService
    pub fn set_port_supported_classof_service(&mut self, value: u32) {
        self.port_supported_classof_service = Some(value);
    }

    /// Gets the value of PortSupportedClassofService
    pub fn get_port_supported_classof_service(&self) -> Option<&u32> {
        self.port_supported_classof_service.as_ref()
    }

    /// Sets the value of PortSupportedFc4Types
    pub fn set_port_supported_fc4_types(&mut self, value: Vec<u8>) {
        self.port_supported_fc4_types = value;
    }

    /// Gets the value of PortSupportedFc4Types
    pub fn get_port_supported_fc4_types(&self) -> &Vec<u8> {
        &self.port_supported_fc4_types
    }

    /// Sets the value of PortSymbolicName
    pub fn set_port_symbolic_name(&mut self, value: String) {
        self.port_symbolic_name = Some(value);
    }

    /// Gets the value of PortSymbolicName
    pub fn get_port_symbolic_name(&self) -> Option<&String> {
        self.port_symbolic_name.as_ref()
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

