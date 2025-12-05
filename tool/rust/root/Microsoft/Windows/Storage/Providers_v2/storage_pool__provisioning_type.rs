// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StoragePool_ProvisioningType
//////////////////////////////////////////////

/// StoragePool_ProvisioningType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StoragePool_ProvisioningType {
    /// Thin
    #[serde(rename = "Thin")]
    Thin = 1,
    /// Fixed
    #[serde(rename = "Fixed")]
    Fixed = 2,
}

impl Default for StoragePool_ProvisioningType {
    fn default() -> Self {
        Self::Thin
    }
}

