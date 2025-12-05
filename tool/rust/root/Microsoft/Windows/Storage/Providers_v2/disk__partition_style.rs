// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Disk_PartitionStyle
//////////////////////////////////////////////

/// Disk_PartitionStyle enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Disk_PartitionStyle {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// MBR
    #[serde(rename = "MBR")]
    MBR = 1,
    /// GPT
    #[serde(rename = "GPT")]
    GPT = 2,
}

impl Default for Disk_PartitionStyle {
    fn default() -> Self {
        Self::Unknown
    }
}

