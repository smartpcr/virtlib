// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Disk_ProvisioningType
//////////////////////////////////////////////

/// Disk_ProvisioningType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Disk_ProvisioningType {
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

impl Default for Disk_ProvisioningType {
    fn default() -> Self {
        Self::Unknown
    }
}

