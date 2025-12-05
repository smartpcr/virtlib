// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Disk_BusType
//////////////////////////////////////////////

/// Disk_BusType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Disk_BusType {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// SCSI
    #[serde(rename = "SCSI")]
    SCSI = 1,
    /// ATAPI
    #[serde(rename = "ATAPI")]
    ATAPI = 2,
    /// ATA
    #[serde(rename = "ATA")]
    ATA = 3,
    /// _1394
    #[serde(rename = "_1394")]
    V1394 = 4,
    /// SSA
    #[serde(rename = "SSA")]
    SSA = 5,
    /// Fibre_Channel
    #[serde(rename = "Fibre_Channel")]
    FibreChannel = 6,
    /// USB
    #[serde(rename = "USB")]
    USB = 7,
    /// RAID
    #[serde(rename = "RAID")]
    RAID = 8,
    /// iSCSI
    #[serde(rename = "iSCSI")]
    ISCSI = 9,
    /// SAS
    #[serde(rename = "SAS")]
    SAS = 10,
    /// SATA
    #[serde(rename = "SATA")]
    SATA = 11,
    /// SD
    #[serde(rename = "SD")]
    SD = 12,
    /// MMC
    #[serde(rename = "MMC")]
    MMC = 13,
    /// Virtual
    #[serde(rename = "Virtual")]
    VirtualValue = 14,
    /// File_Backed_Virtual
    #[serde(rename = "File_Backed_Virtual")]
    FileBackedVirtual = 15,
    /// Storage_Spaces
    #[serde(rename = "Storage_Spaces")]
    StorageSpaces = 16,
    /// NVMe
    #[serde(rename = "NVMe")]
    NVMe = 17,
}

impl Default for Disk_BusType {
    fn default() -> Self {
        Self::Unknown
    }
}

