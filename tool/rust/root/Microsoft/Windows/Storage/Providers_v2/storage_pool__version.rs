// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StoragePool_Version
//////////////////////////////////////////////

/// StoragePool_Version enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StoragePool_Version {
    /// Windows_Server_2012
    #[serde(rename = "Windows_Server_2012")]
    WindowsServer2012 = 1,
    /// Windows_Server_2012_R2_Preview
    #[serde(rename = "Windows_Server_2012_R2_Preview")]
    WindowsServer2012R2Preview = 2,
    /// Windows_Server_2012_R2
    #[serde(rename = "Windows_Server_2012_R2")]
    WindowsServer2012R2 = 3,
    /// Pool_Metadata_Version
    #[serde(rename = "Pool_Metadata_Version")]
    PoolMetadataVersion = 4,
}

impl Default for StoragePool_Version {
    fn default() -> Self {
        Self::WindowsServer2012
    }
}

