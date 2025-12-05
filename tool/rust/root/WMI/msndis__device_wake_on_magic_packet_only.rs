// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_DeviceWakeOnMagicPacketOnly struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_DeviceWakeOnMagicPacketOnly {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "EnableWakeOnMagicPacketOnly")]
    pub enable_wake_on_magic_packet_only: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MSNdis_DeviceWakeOnMagicPacketOnly {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            active: None,
            enable_wake_on_magic_packet_only: None,
            instance_name: None,
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

    /// Sets the value of EnableWakeOnMagicPacketOnly
    pub fn set_enable_wake_on_magic_packet_only(&mut self, value: bool) {
        self.enable_wake_on_magic_packet_only = Some(value);
    }

    /// Gets the value of EnableWakeOnMagicPacketOnly
    pub fn get_enable_wake_on_magic_packet_only(&self) -> Option<&bool> {
        self.enable_wake_on_magic_packet_only.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }
}

