// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapter_VmqFilter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapter_VmqFilter {

/// 
    #[serde(rename = "FilterID")]
    pub filter_id: Option<u32>,

/// 
    #[serde(rename = "MacAddress")]
    pub mac_address: Option<String>,

/// 
    #[serde(rename = "VlanID")]
    pub vlan_id: Option<u16>,
}

impl MSFT_NetAdapter_VmqFilter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            filter_id: None,
            mac_address: None,
            vlan_id: None,
        }
    }


    /// Sets the value of FilterID
    pub fn set_filter_id(&mut self, value: u32) {
        self.filter_id = Some(value);
    }

    /// Gets the value of FilterID
    pub fn get_filter_id(&self) -> Option<&u32> {
        self.filter_id.as_ref()
    }

    /// Sets the value of MacAddress
    pub fn set_mac_address(&mut self, value: String) {
        self.mac_address = Some(value);
    }

    /// Gets the value of MacAddress
    pub fn get_mac_address(&self) -> Option<&String> {
        self.mac_address.as_ref()
    }

    /// Sets the value of VlanID
    pub fn set_vlan_id(&mut self, value: u16) {
        self.vlan_id = Some(value);
    }

    /// Gets the value of VlanID
    pub fn get_vlan_id(&self) -> Option<&u16> {
        self.vlan_id.as_ref()
    }
}

