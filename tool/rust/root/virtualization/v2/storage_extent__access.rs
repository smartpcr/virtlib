// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageExtent_Access
//////////////////////////////////////////////

/// StorageExtent_Access enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageExtent_Access {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Readable
    #[serde(rename = "Readable")]
    Readable = 1,
    /// Writeable
    #[serde(rename = "Writeable")]
    Writeable = 2,
    /// Read_Write_Supported
    #[serde(rename = "Read_Write_Supported")]
    ReadWriteSupported = 3,
    /// Write_Once
    #[serde(rename = "Write_Once")]
    WriteOnce = 4,
}

impl Default for StorageExtent_Access {
    fn default() -> Self {
        Self::Unknown
    }
}

