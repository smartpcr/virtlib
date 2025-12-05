// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HBAFC3MgmtInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HBAFC3MgmtInfo {

/// 
    #[serde(rename = "IPAddress")]
    pub ipaddress: Vec<u8>,

/// 
    #[serde(rename = "IPVersion")]
    pub ipversion: Option<u16>,

/// 
    #[serde(rename = "NumberOfAttachedNodes")]
    pub number_of_attached_nodes: Option<u32>,

/// 
    #[serde(rename = "PortId")]
    pub port_id: Option<u32>,

/// 
    #[serde(rename = "reserved")]
    pub reserved: Option<u16>,

/// 
    #[serde(rename = "reserved1")]
    pub reserved1: Option<u32>,

/// 
    #[serde(rename = "TopologyDiscoveryFlags")]
    pub topology_discovery_flags: Option<u16>,

/// 
    #[serde(rename = "UDPPort")]
    pub udpport: Option<u16>,

/// 
    #[serde(rename = "UniqueAdapterId")]
    pub unique_adapter_id: Option<u64>,

/// 
    #[serde(rename = "unittype")]
    pub unittype: Option<u32>,

/// 
    #[serde(rename = "wwn")]
    pub wwn: Vec<u8>,
}

impl HBAFC3MgmtInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            ipaddress: Vec::new(),
            ipversion: None,
            number_of_attached_nodes: None,
            port_id: None,
            reserved: None,
            reserved1: None,
            topology_discovery_flags: None,
            udpport: None,
            unique_adapter_id: None,
            unittype: None,
            wwn: Vec::new(),
        }
    }


    /// Sets the value of IPAddress
    pub fn set_ipaddress(&mut self, value: Vec<u8>) {
        self.ipaddress = value;
    }

    /// Gets the value of IPAddress
    pub fn get_ipaddress(&self) -> &Vec<u8> {
        &self.ipaddress
    }

    /// Sets the value of IPVersion
    pub fn set_ipversion(&mut self, value: u16) {
        self.ipversion = Some(value);
    }

    /// Gets the value of IPVersion
    pub fn get_ipversion(&self) -> Option<&u16> {
        self.ipversion.as_ref()
    }

    /// Sets the value of NumberOfAttachedNodes
    pub fn set_number_of_attached_nodes(&mut self, value: u32) {
        self.number_of_attached_nodes = Some(value);
    }

    /// Gets the value of NumberOfAttachedNodes
    pub fn get_number_of_attached_nodes(&self) -> Option<&u32> {
        self.number_of_attached_nodes.as_ref()
    }

    /// Sets the value of PortId
    pub fn set_port_id(&mut self, value: u32) {
        self.port_id = Some(value);
    }

    /// Gets the value of PortId
    pub fn get_port_id(&self) -> Option<&u32> {
        self.port_id.as_ref()
    }

    /// Sets the value of reserved
    pub fn set_reserved(&mut self, value: u16) {
        self.reserved = Some(value);
    }

    /// Gets the value of reserved
    pub fn get_reserved(&self) -> Option<&u16> {
        self.reserved.as_ref()
    }

    /// Sets the value of reserved1
    pub fn set_reserved1(&mut self, value: u32) {
        self.reserved1 = Some(value);
    }

    /// Gets the value of reserved1
    pub fn get_reserved1(&self) -> Option<&u32> {
        self.reserved1.as_ref()
    }

    /// Sets the value of TopologyDiscoveryFlags
    pub fn set_topology_discovery_flags(&mut self, value: u16) {
        self.topology_discovery_flags = Some(value);
    }

    /// Gets the value of TopologyDiscoveryFlags
    pub fn get_topology_discovery_flags(&self) -> Option<&u16> {
        self.topology_discovery_flags.as_ref()
    }

    /// Sets the value of UDPPort
    pub fn set_udpport(&mut self, value: u16) {
        self.udpport = Some(value);
    }

    /// Gets the value of UDPPort
    pub fn get_udpport(&self) -> Option<&u16> {
        self.udpport.as_ref()
    }

    /// Sets the value of UniqueAdapterId
    pub fn set_unique_adapter_id(&mut self, value: u64) {
        self.unique_adapter_id = Some(value);
    }

    /// Gets the value of UniqueAdapterId
    pub fn get_unique_adapter_id(&self) -> Option<&u64> {
        self.unique_adapter_id.as_ref()
    }

    /// Sets the value of unittype
    pub fn set_unittype(&mut self, value: u32) {
        self.unittype = Some(value);
    }

    /// Gets the value of unittype
    pub fn get_unittype(&self) -> Option<&u32> {
        self.unittype.as_ref()
    }

    /// Sets the value of wwn
    pub fn set_wwn(&mut self, value: Vec<u8>) {
        self.wwn = value;
    }

    /// Gets the value of wwn
    pub fn get_wwn(&self) -> &Vec<u8> {
        &self.wwn
    }
}

