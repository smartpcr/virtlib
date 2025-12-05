// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_80211_ReceivedSignalStrengthEventTrigger struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_80211_ReceivedSignalStrengthEventTrigger {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "Ndis80211ReceivedSignalStrengthTrigger")]
    pub ndis80211_received_signal_strength_trigger: Option<i32>,
}

impl MSNdis_80211_ReceivedSignalStrengthEventTrigger {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            active: None,
            instance_name: None,
            ndis80211_received_signal_strength_trigger: None,
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

    /// Sets the value of Ndis80211ReceivedSignalStrengthTrigger
    pub fn set_ndis80211_received_signal_strength_trigger(&mut self, value: i32) {
        self.ndis80211_received_signal_strength_trigger = Some(value);
    }

    /// Gets the value of Ndis80211ReceivedSignalStrengthTrigger
    pub fn get_ndis80211_received_signal_strength_trigger(&self) -> Option<&i32> {
        self.ndis80211_received_signal_strength_trigger.as_ref()
    }
}

