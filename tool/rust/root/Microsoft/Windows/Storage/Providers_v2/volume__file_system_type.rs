// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Volume_FileSystemType
//////////////////////////////////////////////

/// Volume_FileSystemType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Volume_FileSystemType {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Threshold
    #[serde(rename = "Threshold")]
    Threshold = 1,
    /// UFS
    #[serde(rename = "UFS")]
    UFS = 2,
    /// HFS
    #[serde(rename = "HFS")]
    HFS = 3,
    /// FAT
    #[serde(rename = "FAT")]
    FAT = 4,
    /// FAT16
    #[serde(rename = "FAT16")]
    FAT16 = 5,
    /// FAT32
    #[serde(rename = "FAT32")]
    FAT32 = 6,
    /// NTFS4
    #[serde(rename = "NTFS4")]
    NTFS4 = 7,
    /// NTFS5
    #[serde(rename = "NTFS5")]
    NTFS5 = 8,
    /// XFS
    #[serde(rename = "XFS")]
    XFS = 9,
    /// AFS
    #[serde(rename = "AFS")]
    AFS = 10,
    /// EXT2
    #[serde(rename = "EXT2")]
    EXT2 = 11,
    /// EXT3
    #[serde(rename = "EXT3")]
    EXT3 = 12,
    /// ReiserFS
    #[serde(rename = "ReiserFS")]
    ReiserFS = 13,
    /// NTFS
    #[serde(rename = "NTFS")]
    NTFS = 14,
    /// ReFS
    #[serde(rename = "ReFS")]
    ReFS = 15,
    /// exFATCSVFS_NTFS
    #[serde(rename = "exFATCSVFS_NTFS")]
    ExFATCSVFSNTFS = 16,
    /// CSVFS_ReFS
    #[serde(rename = "CSVFS_ReFS")]
    CSVFSReFS = 17,
}

impl Default for Volume_FileSystemType {
    fn default() -> Self {
        Self::Unknown
    }
}

