// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFC_HBAPortAttributesResults struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFC_HBAPortAttributesResults {

/// 
    #[serde(rename = "FabricName")]
    pub fabric_name: Vec<u8>,

/// 
    #[serde(rename = "NodeWWN")]
    pub node_wwn: Vec<u8>,

/// 
    #[serde(rename = "NumberofDiscoveredPorts")]
    pub numberof_discovered_ports: Option<u32>,

/// 
    #[serde(rename = "PortActiveFc4Types")]
    pub port_active_fc4_types: Vec<u8>,

/// 
    #[serde(rename = "PortFcId")]
    pub port_fc_id: Option<u32>,

/// 
    #[serde(rename = "PortMaxFrameSize")]
    pub port_max_frame_size: Option<u32>,

/// 
    #[serde(rename = "PortSpeed")]
    pub port_speed: Option<u32>,

/// 
    #[serde(rename = "PortState")]
    pub port_state: Option<u32>,

/// 
    #[serde(rename = "PortSupportedClassofService")]
    pub port_supported_classof_service: Option<u32>,

/// 
    #[serde(rename = "PortSupportedFc4Types")]
    pub port_supported_fc4_types: Vec<u8>,

/// 
    #[serde(rename = "PortSupportedSpeed")]
    pub port_supported_speed: Option<u32>,

/// 
    #[serde(rename = "PortType")]
    pub port_type: Option<u32>,

/// 
    #[serde(rename = "PortWWN")]
    pub port_wwn: Vec<u8>,
}

impl MSFC_HBAPortAttributesResults {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            fabric_name: Vec::new(),
            node_wwn: Vec::new(),
            numberof_discovered_ports: None,
            port_active_fc4_types: Vec::new(),
            port_fc_id: None,
            port_max_frame_size: None,
            port_speed: None,
            port_state: None,
            port_supported_classof_service: None,
            port_supported_fc4_types: Vec::new(),
            port_supported_speed: None,
            port_type: None,
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

    /// Sets the value of PortActiveFc4Types
    pub fn set_port_active_fc4_types(&mut self, value: Vec<u8>) {
        self.port_active_fc4_types = value;
    }

    /// Gets the value of PortActiveFc4Types
    pub fn get_port_active_fc4_types(&self) -> &Vec<u8> {
        &self.port_active_fc4_types
    }

    /// Sets the value of PortFcId
    pub fn set_port_fc_id(&mut self, value: u32) {
        self.port_fc_id = Some(value);
    }

    /// Gets the value of PortFcId
    pub fn get_port_fc_id(&self) -> Option<&u32> {
        self.port_fc_id.as_ref()
    }

    /// Sets the value of PortMaxFrameSize
    pub fn set_port_max_frame_size(&mut self, value: u32) {
        self.port_max_frame_size = Some(value);
    }

    /// Gets the value of PortMaxFrameSize
    pub fn get_port_max_frame_size(&self) -> Option<&u32> {
        self.port_max_frame_size.as_ref()
    }

    /// Sets the value of PortSpeed
    pub fn set_port_speed(&mut self, value: u32) {
        self.port_speed = Some(value);
    }

    /// Gets the value of PortSpeed
    pub fn get_port_speed(&self) -> Option<&u32> {
        self.port_speed.as_ref()
    }

    /// Sets the value of PortState
    pub fn set_port_state(&mut self, value: u32) {
        self.port_state = Some(value);
    }

    /// Gets the value of PortState
    pub fn get_port_state(&self) -> Option<&u32> {
        self.port_state.as_ref()
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

    /// Sets the value of PortSupportedSpeed
    pub fn set_port_supported_speed(&mut self, value: u32) {
        self.port_supported_speed = Some(value);
    }

    /// Gets the value of PortSupportedSpeed
    pub fn get_port_supported_speed(&self) -> Option<&u32> {
        self.port_supported_speed.as_ref()
    }

    /// Sets the value of PortType
    pub fn set_port_type(&mut self, value: u32) {
        self.port_type = Some(value);
    }

    /// Gets the value of PortType
    pub fn get_port_type(&self) -> Option<&u32> {
        self.port_type.as_ref()
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

