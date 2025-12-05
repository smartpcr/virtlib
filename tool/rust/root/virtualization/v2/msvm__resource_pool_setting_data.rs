// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_ResourcePoolSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_ResourcePoolSettingData {
    #[serde(flatten)]
    pub base: Msvm_AbstractResourcePoolSettingData,
}

impl Msvm_ResourcePoolSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_AbstractResourcePoolSettingData::new(),
        }
    }

}

impl Msvm_ResourcePoolSettingData {
    /// Gets the related Msvm_ResourcePool object(s)
    pub fn get_related__resource_pool(&self) -> Result<Msvm_ResourcePool, WmiError> {
        self.get_related("Msvm_ResourcePool")
    }

}

