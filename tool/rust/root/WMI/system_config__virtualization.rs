// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_Virtualization struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_Virtualization {
    #[serde(flatten)]
    pub base: SystemConfig_V2,

/// 
    #[serde(rename = "HvciEnabled")]
    pub hvci_enabled: Option<u8>,

/// 
    #[serde(rename = "HyperVisorEnabled")]
    pub hyper_visor_enabled: Option<u8>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u8>,

/// 
    #[serde(rename = "VbsEnabled")]
    pub vbs_enabled: Option<u8>,
}

impl SystemConfig_Virtualization {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V2::new(),
            hvci_enabled: None,
            hyper_visor_enabled: None,
            reserved: None,
            vbs_enabled: None,
        }
    }


    /// Sets the value of HvciEnabled
    pub fn set_hvci_enabled(&mut self, value: u8) {
        self.hvci_enabled = Some(value);
    }

    /// Gets the value of HvciEnabled
    pub fn get_hvci_enabled(&self) -> Option<&u8> {
        self.hvci_enabled.as_ref()
    }

    /// Sets the value of HyperVisorEnabled
    pub fn set_hyper_visor_enabled(&mut self, value: u8) {
        self.hyper_visor_enabled = Some(value);
    }

    /// Gets the value of HyperVisorEnabled
    pub fn get_hyper_visor_enabled(&self) -> Option<&u8> {
        self.hyper_visor_enabled.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u8) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u8> {
        self.reserved.as_ref()
    }

    /// Sets the value of VbsEnabled
    pub fn set_vbs_enabled(&mut self, value: u8) {
        self.vbs_enabled = Some(value);
    }

    /// Gets the value of VbsEnabled
    pub fn get_vbs_enabled(&self) -> Option<&u8> {
        self.vbs_enabled.as_ref()
    }
}

