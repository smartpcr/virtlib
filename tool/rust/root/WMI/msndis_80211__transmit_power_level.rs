// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_80211_TransmitPowerLevel struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_80211_TransmitPowerLevel {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "Ndis80211TransmitPowerLevel")]
    pub ndis80211_transmit_power_level: Option<u32>,
}

impl MSNdis_80211_TransmitPowerLevel {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            active: None,
            instance_name: None,
            ndis80211_transmit_power_level: None,
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

    /// Sets the value of Ndis80211TransmitPowerLevel
    pub fn set_ndis80211_transmit_power_level(&mut self, value: u32) {
        self.ndis80211_transmit_power_level = Some(value);
    }

    /// Gets the value of Ndis80211TransmitPowerLevel
    pub fn get_ndis80211_transmit_power_level(&self) -> Option<&u32> {
        self.ndis80211_transmit_power_level.as_ref()
    }
}

