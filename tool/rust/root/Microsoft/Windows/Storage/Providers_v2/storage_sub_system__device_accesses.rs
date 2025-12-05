// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageSubSystem_DeviceAccesses
//////////////////////////////////////////////

/// StorageSubSystem_DeviceAccesses enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageSubSystem_DeviceAccesses {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Read_Write
    #[serde(rename = "Read_Write")]
    ReadWrite = 2,
    /// Read_Only
    #[serde(rename = "Read_Only")]
    ReadOnly = 3,
    /// No_Access
    #[serde(rename = "No_Access")]
    NoAccess = 4,
}

impl Default for StorageSubSystem_DeviceAccesses {
    fn default() -> Self {
        Self::Unknown
    }
}

