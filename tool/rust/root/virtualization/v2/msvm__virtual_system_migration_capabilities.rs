// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualSystemMigrationCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualSystemMigrationCapabilities {
    #[serde(flatten)]
    pub base: CIM_VirtualSystemMigrationCapabilities,
}

impl Msvm_VirtualSystemMigrationCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_VirtualSystemMigrationCapabilities::new(),
        }
    }

}

impl Msvm_VirtualSystemMigrationCapabilities {
    /// Gets the related Msvm_VirtualSystemMigrationService object(s)
    pub fn get_related__virtual_system_migration_service(&self) -> Result<Msvm_VirtualSystemMigrationService, WmiError> {
        self.get_related("Msvm_VirtualSystemMigrationService")
    }

    /// Gets the related Msvm_VirtualSystemMigrationSettingData object(s)
    pub fn get_related__virtual_system_migration_setting_data(&self) -> Result<Vec<Msvm_VirtualSystemMigrationSettingData>, WmiError> {
        self.get_all_related("Msvm_VirtualSystemMigrationSettingData")
    }

}

