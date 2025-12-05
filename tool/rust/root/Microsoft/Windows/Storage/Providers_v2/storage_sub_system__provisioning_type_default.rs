// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageSubSystem_ProvisioningTypeDefault
//////////////////////////////////////////////

/// StorageSubSystem_ProvisioningTypeDefault enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageSubSystem_ProvisioningTypeDefault {
    /// Thin
    #[serde(rename = "Thin")]
    Thin = 1,
    /// Fixed
    #[serde(rename = "Fixed")]
    Fixed = 2,
}

impl Default for StorageSubSystem_ProvisioningTypeDefault {
    fn default() -> Self {
        Self::Thin
    }
}

