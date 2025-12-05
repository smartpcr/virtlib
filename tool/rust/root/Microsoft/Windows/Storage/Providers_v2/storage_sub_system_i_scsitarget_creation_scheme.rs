// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageSubSystem_iSCSITargetCreationScheme
//////////////////////////////////////////////

/// StorageSubSystem_iSCSITargetCreationScheme enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageSubSystem_iSCSITargetCreationScheme {
    /// Not_Applicable
    #[serde(rename = "Not_Applicable")]
    NotApplicable = 0,
    /// Not_Supported
    #[serde(rename = "Not_Supported")]
    NotSupported = 1,
    /// Manual
    #[serde(rename = "Manual")]
    Manual = 2,
    /// Auto
    #[serde(rename = "Auto")]
    Auto = 3,
}

impl Default for StorageSubSystem_iSCSITargetCreationScheme {
    fn default() -> Self {
        Self::NotApplicable
    }
}

