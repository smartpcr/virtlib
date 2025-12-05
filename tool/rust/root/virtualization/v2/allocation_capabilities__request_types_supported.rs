// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source AllocationCapabilities_RequestTypesSupported
//////////////////////////////////////////////

/// AllocationCapabilities_RequestTypesSupported enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum AllocationCapabilities_RequestTypesSupported {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Specific
    #[serde(rename = "Specific")]
    Specific = 2,
    /// General
    #[serde(rename = "General")]
    General = 3,
    /// Both
    #[serde(rename = "Both")]
    Both = 4,
    /// DMTF_reserved
    #[serde(rename = "DMTF_reserved")]
    DMTFReserved = 5,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 6,
}

impl Default for AllocationCapabilities_RequestTypesSupported {
    fn default() -> Self {
        Self::Unknown
    }
}

