// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualSystemManagementCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualSystemManagementCapabilities {
    #[serde(flatten)]
    pub base: CIM_VirtualSystemManagementCapabilities,
}

impl Msvm_VirtualSystemManagementCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_VirtualSystemManagementCapabilities::new(),
        }
    }

}

impl Msvm_VirtualSystemManagementCapabilities {
    /// Gets the related Msvm_VirtualSystemManagementService object(s)
    pub fn get_related__virtual_system_management_service(&self) -> Result<Msvm_VirtualSystemManagementService, WmiError> {
        self.get_related("Msvm_VirtualSystemManagementService")
    }

    /// Gets the related Msvm_VirtualSystemSettingData object(s)
    pub fn get_related__virtual_system_setting_data(&self) -> Result<Vec<Msvm_VirtualSystemSettingData>, WmiError> {
        self.get_all_related("Msvm_VirtualSystemSettingData")
    }

}

