// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ManagedSystemElement_OperatingStatus
//////////////////////////////////////////////

/// ManagedSystemElement_OperatingStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ManagedSystemElement_OperatingStatus {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Not_Available
    #[serde(rename = "Not_Available")]
    NotAvailable = 1,
    /// Servicing
    #[serde(rename = "Servicing")]
    Servicing = 2,
    /// Starting
    #[serde(rename = "Starting")]
    Starting = 3,
    /// Stopping
    #[serde(rename = "Stopping")]
    Stopping = 4,
    /// Stopped
    #[serde(rename = "Stopped")]
    Stopped = 5,
    /// Aborted
    #[serde(rename = "Aborted")]
    Aborted = 6,
    /// Dormant
    #[serde(rename = "Dormant")]
    Dormant = 7,
    /// Completed
    #[serde(rename = "Completed")]
    Completed = 8,
    /// Migrating
    #[serde(rename = "Migrating")]
    Migrating = 9,
    /// Emigrating
    #[serde(rename = "Emigrating")]
    Emigrating = 10,
    /// Immigrating
    #[serde(rename = "Immigrating")]
    Immigrating = 11,
    /// Snapshotting
    #[serde(rename = "Snapshotting")]
    Snapshotting = 12,
    /// Shutting_Down
    #[serde(rename = "Shutting_Down")]
    ShuttingDown = 13,
    /// In_Test
    #[serde(rename = "In_Test")]
    InTest = 14,
    /// Transitioning
    #[serde(rename = "Transitioning")]
    Transitioning = 15,
    /// In_Service
    #[serde(rename = "In_Service")]
    InService = 16,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 17,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 18,
}

impl Default for ManagedSystemElement_OperatingStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

