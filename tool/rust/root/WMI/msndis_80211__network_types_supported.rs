// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_80211_NetworkTypesSupported struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_80211_NetworkTypesSupported {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "Ndis80211NetworkTypes")]
    pub ndis80211_network_types: Vec<MSNdis_80211_NetworkType>,

/// 
    #[serde(rename = "NumberOfItems")]
    pub number_of_items: Option<u32>,
}

impl MSNdis_80211_NetworkTypesSupported {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            active: None,
            instance_name: None,
            ndis80211_network_types: Vec::new(),
            number_of_items: None,
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

    /// Sets the value of Ndis80211NetworkTypes
    pub fn set_ndis80211_network_types(&mut self, value: Vec<MSNdis_80211_NetworkType>) {
        self.ndis80211_network_types = value;
    }

    /// Gets the value of Ndis80211NetworkTypes
    pub fn get_ndis80211_network_types(&self) -> &Vec<MSNdis_80211_NetworkType> {
        &self.ndis80211_network_types
    }

    /// Sets the value of NumberOfItems
    pub fn set_number_of_items(&mut self, value: u32) {
        self.number_of_items = Some(value);
    }

    /// Gets the value of NumberOfItems
    pub fn get_number_of_items(&self) -> Option<&u32> {
        self.number_of_items.as_ref()
    }
}

