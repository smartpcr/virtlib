// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StoragePool_RetireMissingPhysicalDisks
//////////////////////////////////////////////

/// StoragePool_RetireMissingPhysicalDisks enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StoragePool_RetireMissingPhysicalDisks {
    /// Auto
    #[serde(rename = "Auto")]
    Auto = 1,
    /// Always
    #[serde(rename = "Always")]
    Always = 2,
    /// Never
    #[serde(rename = "Never")]
    Never = 3,
}

impl Default for StoragePool_RetireMissingPhysicalDisks {
    fn default() -> Self {
        Self::Auto
    }
}

