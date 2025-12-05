// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageNodeToStorageEnclosure_TemperatureSensorOperationalStatus
//////////////////////////////////////////////

/// StorageNodeToStorageEnclosure_TemperatureSensorOperationalStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageNodeToStorageEnclosure_TemperatureSensorOperationalStatus {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// OK
    #[serde(rename = "OK")]
    OK = 2,
    /// Degraded
    #[serde(rename = "Degraded")]
    Degraded = 3,
    /// Error
    #[serde(rename = "Error")]
    Error = 6,
    /// Non_Recoverable_Error
    #[serde(rename = "Non_Recoverable_Error")]
    NonRecoverableError = 7,
    /// Not_Installed
    #[serde(rename = "Not_Installed")]
    NotInstalled = 8,
    /// Not_Available
    #[serde(rename = "Not_Available")]
    NotAvailable = 9,
    /// No_Access_Allowed
    #[serde(rename = "No_Access_Allowed")]
    NoAccessAllowed = 10,
    /// Not_Reported
    #[serde(rename = "Not_Reported")]
    NotReported = 11,
}

impl Default for StorageNodeToStorageEnclosure_TemperatureSensorOperationalStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

