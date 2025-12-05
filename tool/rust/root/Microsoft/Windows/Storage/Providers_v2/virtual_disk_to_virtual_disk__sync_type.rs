// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualDiskToVirtualDisk_SyncType
//////////////////////////////////////////////

/// VirtualDiskToVirtualDisk_SyncType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualDiskToVirtualDisk_SyncType {
    /// Microsoft_Reserved
    #[serde(rename = "Microsoft_Reserved")]
    MicrosoftReserved = 1,
    /// Mirror
    #[serde(rename = "Mirror")]
    Mirror = 6,
    /// Snapshot
    #[serde(rename = "Snapshot")]
    Snapshot = 7,
    /// Clone
    #[serde(rename = "Clone")]
    Clone = 8,
    /// Microsoft_Reserved1
    #[serde(rename = "Microsoft_Reserved1")]
    MicrosoftReserved1 = 9,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 10,
}

impl Default for VirtualDiskToVirtualDisk_SyncType {
    fn default() -> Self {
        Self::MicrosoftReserved
    }
}

