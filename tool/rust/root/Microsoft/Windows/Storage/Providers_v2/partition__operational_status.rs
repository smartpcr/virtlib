// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Partition_OperationalStatus
//////////////////////////////////////////////

/// Partition_OperationalStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Partition_OperationalStatus {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Online
    #[serde(rename = "Online")]
    Online = 1,
    /// No_Media
    #[serde(rename = "No_Media")]
    NoMedia = 3,
    /// Failed
    #[serde(rename = "Failed")]
    Failed = 5,
    /// Offline
    #[serde(rename = "Offline")]
    Offline = 4,
}

impl Default for Partition_OperationalStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

