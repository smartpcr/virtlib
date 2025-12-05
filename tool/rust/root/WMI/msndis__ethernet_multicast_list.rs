// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_EthernetMulticastList struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_EthernetMulticastList {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "NdisMulticastList")]
    pub ndis_multicast_list: Vec<MSNdis_NetworkAddress>,

/// 
    #[serde(rename = "NumberElements")]
    pub number_elements: Option<u32>,
}

impl MSNdis_EthernetMulticastList {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            active: None,
            instance_name: None,
            ndis_multicast_list: Vec::new(),
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

    /// Sets the value of NdisMulticastList
    pub fn set_ndis_multicast_list(&mut self, value: Vec<MSNdis_NetworkAddress>) {
        self.ndis_multicast_list = value;
    }

    /// Gets the value of NdisMulticastList
    pub fn get_ndis_multicast_list(&self) -> &Vec<MSNdis_NetworkAddress> {
        &self.ndis_multicast_list
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

