// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_EthernetSwitchOperationalData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_EthernetSwitchOperationalData {
    #[serde(flatten)]
    pub base: Msvm_EthernetSwitchData,

/// 
    #[serde(rename = "CurrentSwitchingMode")]
    pub current_switching_mode: Option<u32>,

/// 
    #[serde(rename = "SupportedSwitchingModes")]
    pub supported_switching_modes: Vec<u32>,
}

impl Msvm_EthernetSwitchOperationalData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_EthernetSwitchData::new(),
            current_switching_mode: None,
            supported_switching_modes: Vec::new(),
        }
    }


    /// Sets the value of CurrentSwitchingMode
    pub fn set_current_switching_mode(&mut self, value: u32) {
        self.current_switching_mode = Some(value);
    }

    /// Gets the value of CurrentSwitchingMode
    pub fn get_current_switching_mode(&self) -> Option<&u32> {
        self.current_switching_mode.as_ref()
    }

    /// Sets the value of SupportedSwitchingModes
    pub fn set_supported_switching_modes(&mut self, value: Vec<u32>) {
        self.supported_switching_modes = value;
    }

    /// Gets the value of SupportedSwitchingModes
    pub fn get_supported_switching_modes(&self) -> &Vec<u32> {
        &self.supported_switching_modes
    }
}

impl Msvm_EthernetSwitchOperationalData {
    /// Gets the related Msvm_VirtualEthernetSwitch object(s)
    pub fn get_related__virtual_ethernet_switch(&self) -> Result<Msvm_VirtualEthernetSwitch, WmiError> {
        self.get_related("Msvm_VirtualEthernetSwitch")
    }

}

