// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_ResourceAllocationSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_ResourceAllocationSettingData {
    #[serde(flatten)]
    pub base: CIM_ResourceAllocationSettingData,

/// 
    #[serde(rename = "TargetVtl")]
    pub target_vtl: Option<u8>,

/// 
    #[serde(rename = "VirtualSystemIdentifiers")]
    pub virtual_system_identifiers: Vec<String>,
}

impl Msvm_ResourceAllocationSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ResourceAllocationSettingData::new(),
            target_vtl: None,
            virtual_system_identifiers: Vec::new(),
        }
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

impl Msvm_ResourceAllocationSettingData {
    /// Gets the related Msvm_AllocationCapabilities object(s)
    pub fn get_related__allocation_capabilities(&self) -> Result<Msvm_AllocationCapabilities, WmiError> {
        self.get_related("Msvm_AllocationCapabilities")
    }

}

