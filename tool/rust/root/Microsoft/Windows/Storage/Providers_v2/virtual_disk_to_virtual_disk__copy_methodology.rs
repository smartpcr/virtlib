// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualDiskToVirtualDisk_CopyMethodology
//////////////////////////////////////////////

/// VirtualDiskToVirtualDisk_CopyMethodology enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualDiskToVirtualDisk_CopyMethodology {
    /// Not_Specified
    #[serde(rename = "Not_Specified")]
    NotSpecified = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Implementation_decides
    #[serde(rename = "Implementation_decides")]
    ImplementationDecides = 2,
    /// Full_Copy
    #[serde(rename = "Full_Copy")]
    FullCopy = 3,
    /// Incremental_Copy
    #[serde(rename = "Incremental_Copy")]
    IncrementalCopy = 4,
    /// Differential_Copy
    #[serde(rename = "Differential_Copy")]
    DifferentialCopy = 5,
    /// Copy_On_Write
    #[serde(rename = "Copy_On_Write")]
    CopyOnWrite = 6,
    /// Copy_On_Access
    #[serde(rename = "Copy_On_Access")]
    CopyOnAccess = 7,
    /// Delta_Update
    #[serde(rename = "Delta_Update")]
    DeltaUpdate = 8,
    /// Snap_And_Clone
    #[serde(rename = "Snap_And_Clone")]
    SnapAndClone = 9,
    /// Microsoft_Reserved
    #[serde(rename = "Microsoft_Reserved")]
    MicrosoftReserved = 10,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 11,
}

impl Default for VirtualDiskToVirtualDisk_CopyMethodology {
    fn default() -> Self {
        Self::NotSpecified
    }
}

