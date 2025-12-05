// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualDiskToVirtualDisk_CopyType
//////////////////////////////////////////////

/// VirtualDiskToVirtualDisk_CopyType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualDiskToVirtualDisk_CopyType {
    /// Async
    #[serde(rename = "Async")]
    AsyncValue = 2,
    /// Sync
    #[serde(rename = "Sync")]
    Sync = 3,
    /// UnSyncAssoc
    #[serde(rename = "UnSyncAssoc")]
    UnSyncAssoc = 4,
    /// UnSyncUnAssoc
    #[serde(rename = "UnSyncUnAssoc")]
    UnSyncUnAssoc = 5,
    /// Microsoft_Reserved
    #[serde(rename = "Microsoft_Reserved")]
    MicrosoftReserved = 6,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 7,
}

impl Default for VirtualDiskToVirtualDisk_CopyType {
    fn default() -> Self {
        Self::AsyncValue
    }
}

