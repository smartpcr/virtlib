// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_FlexIoDeviceSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_FlexIoDeviceSettingData {
    #[serde(flatten)]
    pub base: CIM_ResourceAllocationSettingData,

/// 
    #[serde(rename = "EmulatorConfiguration")]
    pub emulator_configuration: Vec<String>,

/// 
    #[serde(rename = "EmulatorId")]
    pub emulator_id: Option<String>,

/// 
    #[serde(rename = "PhysicalNumaNode")]
    pub physical_numa_node: Option<u16>,

/// 
    #[serde(rename = "TargetVtl")]
    pub target_vtl: Option<u8>,

/// 
    #[serde(rename = "VirtualSystemIdentifiers")]
    pub virtual_system_identifiers: Vec<String>,
}

impl Msvm_FlexIoDeviceSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ResourceAllocationSettingData::new(),
            emulator_configuration: Vec::new(),
            emulator_id: None,
            physical_numa_node: None,
            target_vtl: None,
            virtual_system_identifiers: Vec::new(),
        }
    }


    /// Sets the value of EmulatorConfiguration
    pub fn set_emulator_configuration(&mut self, value: Vec<String>) {
        self.emulator_configuration = value;
    }

    /// Gets the value of EmulatorConfiguration
    pub fn get_emulator_configuration(&self) -> &Vec<String> {
        &self.emulator_configuration
    }

    /// Sets the value of EmulatorId
    pub fn set_emulator_id(&mut self, value: String) {
        self.emulator_id = Some(value);
    }

    /// Gets the value of EmulatorId
    pub fn get_emulator_id(&self) -> Option<&String> {
        self.emulator_id.as_ref()
    }

    /// Sets the value of PhysicalNumaNode
    pub fn set_physical_numa_node(&mut self, value: u16) {
        self.physical_numa_node = Some(value);
    }

    /// Gets the value of PhysicalNumaNode
    pub fn get_physical_numa_node(&self) -> Option<&u16> {
        self.physical_numa_node.as_ref()
    }

    /// Sets the value of TargetVtl
    pub fn set_target_vtl(&mut self, value: u8) {
        self.target_vtl = Some(value);
    }

    /// Gets the value of TargetVtl
    pub fn get_target_vtl(&self) -> Option<&u8> {
        self.target_vtl.as_ref()
    }

    /// Sets the value of VirtualSystemIdentifiers
    pub fn set_virtual_system_identifiers(&mut self, value: Vec<String>) {
        self.virtual_system_identifiers = value;
    }

    /// Gets the value of VirtualSystemIdentifiers
    pub fn get_virtual_system_identifiers(&self) -> &Vec<String> {
        &self.virtual_system_identifiers
    }
}

impl Msvm_FlexIoDeviceSettingData {
    /// Gets the related Msvm_AllocationCapabilities object(s)
    pub fn get_related__allocation_capabilities(&self) -> Result<Msvm_AllocationCapabilities, WmiError> {
        self.get_related("Msvm_AllocationCapabilities")
    }

}

