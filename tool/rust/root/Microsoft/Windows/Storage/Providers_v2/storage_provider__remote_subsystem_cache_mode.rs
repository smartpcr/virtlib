// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageProvider_RemoteSubsystemCacheMode
//////////////////////////////////////////////

/// StorageProvider_RemoteSubsystemCacheMode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageProvider_RemoteSubsystemCacheMode {
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled = 2,
    /// Manual_Discovery
    #[serde(rename = "Manual_Discovery")]
    ManualDiscovery = 3,
}

impl Default for StorageProvider_RemoteSubsystemCacheMode {
    fn default() -> Self {
        Self::Disabled
    }
}

