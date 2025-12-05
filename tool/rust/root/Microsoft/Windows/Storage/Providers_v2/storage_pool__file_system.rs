// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StoragePool_FileSystem
//////////////////////////////////////////////

/// StoragePool_FileSystem enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StoragePool_FileSystem {
    /// NTFS
    #[serde(rename = "NTFS")]
    NTFS = 14,
    /// ReFS
    #[serde(rename = "ReFS")]
    ReFS = 15,
    /// CSVFS_NTFS
    #[serde(rename = "CSVFS_NTFS")]
    CSVFSNTFS = 16,
    /// CSVFS_ReFS
    #[serde(rename = "CSVFS_ReFS")]
    CSVFSReFS = 17,
}

impl Default for StoragePool_FileSystem {
    fn default() -> Self {
        Self::NTFS
    }
}

