// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualLogicalUnitSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualLogicalUnitSettingData {
    #[serde(flatten)]
    pub base: CIM_ResourceAllocationSettingData,

/// 
    #[serde(rename = "StorageSubsystemType")]
    pub storage_subsystem_type: Option<String>,
}

impl Msvm_VirtualLogicalUnitSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ResourceAllocationSettingData::new(),
            storage_subsystem_type: None,
        }
    }


    /// Sets the value of StorageSubsystemType
    pub fn set_storage_subsystem_type(&mut self, value: String) {
        self.storage_subsystem_type = Some(value);
    }

    /// Gets the value of StorageSubsystemType
    pub fn get_storage_subsystem_type(&self) -> Option<&String> {
        self.storage_subsystem_type.as_ref()
    }
}

impl Msvm_VirtualLogicalUnitSettingData {
    /// Gets the related Msvm_AllocationCapabilities object(s)
    pub fn get_related__allocation_capabilities(&self) -> Result<Msvm_AllocationCapabilities, WmiError> {
        self.get_related("Msvm_AllocationCapabilities")
    }

}

