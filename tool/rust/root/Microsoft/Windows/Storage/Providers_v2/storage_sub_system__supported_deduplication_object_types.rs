// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageSubSystem_SupportedDeduplicationObjectTypes
//////////////////////////////////////////////

/// StorageSubSystem_SupportedDeduplicationObjectTypes enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageSubSystem_SupportedDeduplicationObjectTypes {
    /// Volume
    #[serde(rename = "Volume")]
    Volume = 2,
    /// VirtualDisk
    #[serde(rename = "VirtualDisk")]
    VirtualDisk = 4,
    /// Partition
    #[serde(rename = "Partition")]
    Partition = 8,
    /// StoragePool
    #[serde(rename = "StoragePool")]
    StoragePool = 16,
}

impl Default for StorageSubSystem_SupportedDeduplicationObjectTypes {
    fn default() -> Self {
        Self::Volume
    }
}

