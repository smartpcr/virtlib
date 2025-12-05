// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_ExternalEthernetPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_ExternalEthernetPort {
    #[serde(flatten)]
    pub base: CIM_EthernetPort,

/// 
    #[serde(rename = "IsBound")]
    pub is_bound: Option<bool>,
}

impl Msvm_ExternalEthernetPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_EthernetPort::new(),
            is_bound: None,
        }
    }


    /// Sets the value of IsBound
    pub fn set_is_bound(&mut self, value: bool) {
        self.is_bound = Some(value);
    }

    /// Gets the value of IsBound
    pub fn get_is_bound(&self) -> Option<&bool> {
        self.is_bound.as_ref()
    }
}

impl Msvm_ExternalEthernetPort {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

    /// Gets the related Msvm_ExternalEthernetPortCapabilities object(s)
    pub fn get_related__external_ethernet_port_capabilities(&self) -> Result<Msvm_ExternalEthernetPortCapabilities, WmiError> {
        self.get_related("Msvm_ExternalEthernetPortCapabilities")
    }

}

