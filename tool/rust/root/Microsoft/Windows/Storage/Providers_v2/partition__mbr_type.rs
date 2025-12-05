// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Partition_MbrType
//////////////////////////////////////////////

/// Partition_MbrType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Partition_MbrType {
    /// FAT12
    #[serde(rename = "FAT12")]
    FAT12 = 1,
    /// FAT16
    #[serde(rename = "FAT16")]
    FAT16 = 4,
    /// Extended
    #[serde(rename = "Extended")]
    Extended = 5,
    /// Huge
    #[serde(rename = "Huge")]
    Huge = 6,
    /// IFS
    #[serde(rename = "IFS")]
    IFS = 7,
    /// FAT32
    #[serde(rename = "FAT32")]
    FAT32 = 12,
}

impl Default for Partition_MbrType {
    fn default() -> Self {
        Self::FAT12
    }
}

