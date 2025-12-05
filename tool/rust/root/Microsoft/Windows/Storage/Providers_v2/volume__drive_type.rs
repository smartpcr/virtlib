// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Volume_DriveType
//////////////////////////////////////////////

/// Volume_DriveType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Volume_DriveType {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Invalid_Root_Path
    #[serde(rename = "Invalid_Root_Path")]
    InvalidRootPath = 1,
    /// Removable
    #[serde(rename = "Removable")]
    Removable = 2,
    /// Fixed
    #[serde(rename = "Fixed")]
    Fixed = 3,
    /// Remote
    #[serde(rename = "Remote")]
    Remote = 4,
    /// CD_ROM
    #[serde(rename = "CD_ROM")]
    CDROM = 5,
    /// RAM_Disk
    #[serde(rename = "RAM_Disk")]
    RAMDisk = 6,
}

impl Default for Volume_DriveType {
    fn default() -> Self {
        Self::Unknown
    }
}

