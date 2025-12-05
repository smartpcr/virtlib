// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_80211_NetworkTypeInUse struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_80211_NetworkTypeInUse {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "Ndis80211NetworkTypeInUse")]
    pub ndis80211_network_type_in_use: Option<MSNdis_80211_NetworkType>,
}

impl MSNdis_80211_NetworkTypeInUse {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            active: None,
            instance_name: None,
            ndis80211_network_type_in_use: None,
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

    /// Sets the value of Ndis80211NetworkTypeInUse
    pub fn set_ndis80211_network_type_in_use(&mut self, value: MSNdis_80211_NetworkType) {
        self.ndis80211_network_type_in_use = Some(value);
    }

    /// Gets the value of Ndis80211NetworkTypeInUse
    pub fn get_ndis80211_network_type_in_use(&self) -> Option<&MSNdis_80211_NetworkType> {
        self.ndis80211_network_type_in_use.as_ref()
    }
}

