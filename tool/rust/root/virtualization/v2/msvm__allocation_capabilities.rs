// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_AllocationCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_AllocationCapabilities {
    #[serde(flatten)]
    pub base: CIM_AllocationCapabilities,
}

impl Msvm_AllocationCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_AllocationCapabilities::new(),
        }
    }

}

impl Msvm_AllocationCapabilities {
    /// Gets the related Msvm_ResourcePool object(s)
    pub fn get_related__resource_pool(&self) -> Result<Msvm_ResourcePool, WmiError> {
        self.get_related("Msvm_ResourcePool")
    }

    /// Gets the related Msvm_ResourceAllocationSettingData object(s)
    pub fn get_related__resource_allocation_setting_data(&self) -> Result<Vec<Msvm_ResourceAllocationSettingData>, WmiError> {
        self.get_all_related("Msvm_ResourceAllocationSettingData")
    }

}

