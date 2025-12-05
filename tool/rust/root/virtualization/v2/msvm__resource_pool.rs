// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_ResourcePool struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_ResourcePool {
    #[serde(flatten)]
    pub base: CIM_ResourcePool,
}

impl Msvm_ResourcePool {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ResourcePool::new(),
        }
    }

}

impl Msvm_ResourcePool {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

    /// Gets the related Msvm_ResourcePoolSettingData object(s)
    pub fn get_related__resource_pool_setting_data(&self) -> Result<Msvm_ResourcePoolSettingData, WmiError> {
        self.get_related("Msvm_ResourcePoolSettingData")
    }

    /// Gets the related Msvm_ResourcePoolConfigurationService object(s)
    pub fn get_related__resource_pool_configuration_service(&self) -> Result<Msvm_ResourcePoolConfigurationService, WmiError> {
        self.get_related("Msvm_ResourcePoolConfigurationService")
    }

    /// Gets the related Msvm_AllocationCapabilities object(s)
    pub fn get_related__allocation_capabilities(&self) -> Result<Msvm_AllocationCapabilities, WmiError> {
        self.get_related("Msvm_AllocationCapabilities")
    }

}

