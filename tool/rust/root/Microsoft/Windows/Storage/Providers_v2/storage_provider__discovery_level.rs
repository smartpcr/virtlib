// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageProvider_DiscoveryLevel
//////////////////////////////////////////////

/// StorageProvider_DiscoveryLevel enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageProvider_DiscoveryLevel {
    /// Level_0
    #[serde(rename = "Level_0")]
    Level0 = 0,
    /// Level_1
    #[serde(rename = "Level_1")]
    Level1 = 1,
    /// Level_2
    #[serde(rename = "Level_2")]
    Level2 = 2,
    /// Level_3
    #[serde(rename = "Level_3")]
    Level3 = 3,
}

impl Default for StorageProvider_DiscoveryLevel {
    fn default() -> Self {
        Self::Level0
    }
}

