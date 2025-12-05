// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source DVDDrive_FormatsSupported
//////////////////////////////////////////////

/// DVDDrive_FormatsSupported enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum DVDDrive_FormatsSupported {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// CD_ROM
    #[serde(rename = "CD_ROM")]
    CDROM = 16,
    /// CD_ROM_XA
    #[serde(rename = "CD_ROM_XA")]
    CDROMXA = 17,
    /// CD_I
    #[serde(rename = "CD_I")]
    CDI = 18,
    /// CD_Recordable
    #[serde(rename = "CD_Recordable")]
    CDRecordable = 19,
    /// DVD
    #[serde(rename = "DVD")]
    DVD = 22,
    /// DVD_RWplus
    #[serde(rename = "DVD_RWplus")]
    DVDRWplus = 23,
    /// DVD_RAM
    #[serde(rename = "DVD_RAM")]
    DVDRAM = 24,
    /// DVD_ROM
    #[serde(rename = "DVD_ROM")]
    DVDROM = 25,
    /// DVD_Video
    #[serde(rename = "DVD_Video")]
    DVDVideo = 26,
    /// Divx
    #[serde(rename = "Divx")]
    Divx = 27,
    /// CD_RW
    #[serde(rename = "CD_RW")]
    CDRW = 33,
    /// CD_DA
    #[serde(rename = "CD_DA")]
    CDDA = 34,
    /// CDplus
    #[serde(rename = "CDplus")]
    CDplus = 35,
    /// DVD_Recordable
    #[serde(rename = "DVD_Recordable")]
    DVDRecordable = 36,
    /// DVD_RW
    #[serde(rename = "DVD_RW")]
    DVDRW = 37,
    /// DVD_Audio
    #[serde(rename = "DVD_Audio")]
    DVDAudio = 38,
    /// DVD_5
    #[serde(rename = "DVD_5")]
    DVD5 = 39,
    /// DVD_9
    #[serde(rename = "DVD_9")]
    DVD9 = 40,
    /// DVD_10
    #[serde(rename = "DVD_10")]
    DVD10 = 41,
    /// DVD_18
    #[serde(rename = "DVD_18")]
    DVD18 = 42,
}

impl Default for DVDDrive_FormatsSupported {
    fn default() -> Self {
        Self::Unknown
    }
}

