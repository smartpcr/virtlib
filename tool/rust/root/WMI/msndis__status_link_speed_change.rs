// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_StatusLinkSpeedChange struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_StatusLinkSpeedChange {
    #[serde(flatten)]
    pub base: WMIEvent,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "NdisStatusLinkSpeedChange")]
    pub ndis_status_link_speed_change: Option<MSNdis_NetworkLinkSpeed>,
}

impl MSNdis_StatusLinkSpeedChange {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WMIEvent::new(),
            active: None,
            instance_name: None,
            ndis_status_link_speed_change: None,
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

    /// Sets the value of NdisStatusLinkSpeedChange
    pub fn set_ndis_status_link_speed_change(&mut self, value: MSNdis_NetworkLinkSpeed) {
        self.ndis_status_link_speed_change = Some(value);
    }

    /// Gets the value of NdisStatusLinkSpeedChange
    pub fn get_ndis_status_link_speed_change(&self) -> Option<&MSNdis_NetworkLinkSpeed> {
        self.ndis_status_link_speed_change.as_ref()
    }
}

