// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_VirtualEthernetSwitchSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_VirtualEthernetSwitchSettingData {
    #[serde(flatten)]
    pub base: CIM_VirtualSystemSettingData,

/// A list of host resource pools to be associated or that are currently associated with the Ethernet Switch for the purpose of the allocation of Ethernet connections between a virtual machine and an Ethernet switch. Each non-Null value of the AssociatedResourcePool property shall conform to the production WBEM_URI_UntypedInstancePath as defined in DSP0207.
    #[serde(rename = "AssociatedResourcePool")]
    pub associated_resource_pool: Vec<String>,

/// This property specifies the number of unique MAC addresses that can be learned by the switch to support MAC Address Learning, as defined in the IEEE 802.1 standard.
    #[serde(rename = "MaxNumMACAddress")]
    pub max_num_macaddress: Option<u32>,

/// A list of VLAN Ids that this switch can access.
    #[serde(rename = "VLANConnection")]
    pub vlanconnection: Vec<String>,
}

impl CIM_VirtualEthernetSwitchSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_VirtualSystemSettingData::new(),
            associated_resource_pool: Vec::new(),
            max_num_macaddress: None,
            vlanconnection: Vec::new(),
        }
    }


    /// Sets the value of AssociatedResourcePool
    pub fn set_associated_resource_pool(&mut self, value: Vec<String>) {
        self.associated_resource_pool = value;
    }

    /// Gets the value of AssociatedResourcePool
    pub fn get_associated_resource_pool(&self) -> &Vec<String> {
        &self.associated_resource_pool
    }

    /// Sets the value of MaxNumMACAddress
    pub fn set_max_num_macaddress(&mut self, value: u32) {
        self.max_num_macaddress = Some(value);
    }

    /// Gets the value of MaxNumMACAddress
    pub fn get_max_num_macaddress(&self) -> Option<&u32> {
        self.max_num_macaddress.as_ref()
    }

    /// Sets the value of VLANConnection
    pub fn set_vlanconnection(&mut self, value: Vec<String>) {
        self.vlanconnection = value;
    }

    /// Gets the value of VLANConnection
    pub fn get_vlanconnection(&self) -> &Vec<String> {
        &self.vlanconnection
    }
}

