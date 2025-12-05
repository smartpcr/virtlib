// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_InternalEthernetPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_InternalEthernetPort {
    #[serde(flatten)]
    pub base: CIM_EthernetPort,
}

impl Msvm_InternalEthernetPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_EthernetPort::new(),
        }
    }

}

impl Msvm_InternalEthernetPort {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

    /// Gets the related Msvm_LANEndpoint object(s)
    pub fn get_related__lanendpoint(&self) -> Result<Msvm_LANEndpoint, WmiError> {
        self.get_related("Msvm_LANEndpoint")
    }

}

