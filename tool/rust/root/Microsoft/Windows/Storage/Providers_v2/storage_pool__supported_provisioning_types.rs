// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StoragePool_SupportedProvisioningTypes
//////////////////////////////////////////////

/// StoragePool_SupportedProvisioningTypes enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StoragePool_SupportedProvisioningTypes {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Thin
    #[serde(rename = "Thin")]
    Thin = 1,
    /// Fixed
    #[serde(rename = "Fixed")]
    Fixed = 2,
}

impl Default for StoragePool_SupportedProvisioningTypes {
    fn default() -> Self {
        Self::Unknown
    }
}

