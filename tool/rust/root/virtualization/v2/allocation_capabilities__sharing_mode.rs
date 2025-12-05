// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source AllocationCapabilities_SharingMode
//////////////////////////////////////////////

/// AllocationCapabilities_SharingMode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum AllocationCapabilities_SharingMode {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Dedicated
    #[serde(rename = "Dedicated")]
    Dedicated = 2,
    /// Shared
    #[serde(rename = "Shared")]
    Shared = 3,
    /// DMTF_reserved
    #[serde(rename = "DMTF_reserved")]
    DMTFReserved = 4,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 5,
}

impl Default for AllocationCapabilities_SharingMode {
    fn default() -> Self {
        Self::Unknown
    }
}

