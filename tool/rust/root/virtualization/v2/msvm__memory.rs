// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_Memory struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_Memory {
    #[serde(flatten)]
    pub base: CIM_Memory,

/// 
    #[serde(rename = "MemoryEncryption")]
    pub memory_encryption: Option<bool>,
}

impl Msvm_Memory {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Memory::new(),
            memory_encryption: None,
        }
    }


    /// Sets the value of MemoryEncryption
    pub fn set_memory_encryption(&mut self, value: bool) {
        self.memory_encryption = Some(value);
    }

    /// Gets the value of MemoryEncryption
    pub fn get_memory_encryption(&self) -> Option<&bool> {
        self.memory_encryption.as_ref()
    }
}

impl Msvm_Memory {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

    /// Gets the related Msvm_NumaNode object(s)
    pub fn get_related__numa_node(&self) -> Result<Msvm_NumaNode, WmiError> {
        self.get_related("Msvm_NumaNode")
    }

    /// Gets the related Msvm_ResourcePool object(s)
    pub fn get_related__resource_pool(&self) -> Result<Msvm_ResourcePool, WmiError> {
        self.get_related("Msvm_ResourcePool")
    }

}

