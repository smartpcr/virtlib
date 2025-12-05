// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemSnapshotService_SnapshotType
//////////////////////////////////////////////

/// VirtualSystemSnapshotService_SnapshotType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemSnapshotService_SnapshotType {
    /// Full_Snapshot
    #[serde(rename = "Full_Snapshot")]
    FullSnapshot = 2,
    /// Disk_Snapshot
    #[serde(rename = "Disk_Snapshot")]
    DiskSnapshot = 3,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 4,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 5,
}

impl Default for VirtualSystemSnapshotService_SnapshotType {
    fn default() -> Self {
        Self::FullSnapshot
    }
}

