// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source CollectionSnapshotService_SnapshotType
//////////////////////////////////////////////

/// CollectionSnapshotService_SnapshotType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum CollectionSnapshotService_SnapshotType {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Standard_Snapshot
    #[serde(rename = "Standard_Snapshot")]
    StandardSnapshot = 1,
    /// Recovery_Snapshot
    #[serde(rename = "Recovery_Snapshot")]
    RecoverySnapshot = 2,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 3,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 4,
}

impl Default for CollectionSnapshotService_SnapshotType {
    fn default() -> Self {
        Self::Unknown
    }
}

