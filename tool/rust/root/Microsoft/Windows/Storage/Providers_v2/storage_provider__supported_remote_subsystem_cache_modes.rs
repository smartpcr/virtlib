// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageProvider_SupportedRemoteSubsystemCacheModes
//////////////////////////////////////////////

/// StorageProvider_SupportedRemoteSubsystemCacheModes enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageProvider_SupportedRemoteSubsystemCacheModes {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled = 2,
    /// Manual_Discovery
    #[serde(rename = "Manual_Discovery")]
    ManualDiscovery = 3,
}

impl Default for StorageProvider_SupportedRemoteSubsystemCacheModes {
    fn default() -> Self {
        Self::Unknown
    }
}

