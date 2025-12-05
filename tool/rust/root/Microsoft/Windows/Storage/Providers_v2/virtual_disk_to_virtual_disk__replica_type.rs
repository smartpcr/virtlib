// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualDiskToVirtualDisk_ReplicaType
//////////////////////////////////////////////

/// VirtualDiskToVirtualDisk_ReplicaType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualDiskToVirtualDisk_ReplicaType {
    /// Not_Specified
    #[serde(rename = "Not_Specified")]
    NotSpecified = 0,
    /// Full_Copy
    #[serde(rename = "Full_Copy")]
    FullCopy = 2,
    /// Before_Delta
    #[serde(rename = "Before_Delta")]
    BeforeDelta = 3,
    /// After_Delta
    #[serde(rename = "After_Delta")]
    AfterDelta = 4,
    /// Log
    #[serde(rename = "Log")]
    Log = 5,
    /// Microsoft_Reserved
    #[serde(rename = "Microsoft_Reserved")]
    MicrosoftReserved = 6,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 7,
}

impl Default for VirtualDiskToVirtualDisk_ReplicaType {
    fn default() -> Self {
        Self::NotSpecified
    }
}

