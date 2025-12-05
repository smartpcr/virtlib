// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ManagedSystemElement_PrimaryStatus
//////////////////////////////////////////////

/// ManagedSystemElement_PrimaryStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ManagedSystemElement_PrimaryStatus {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// OK
    #[serde(rename = "OK")]
    OK = 1,
    /// Degraded
    #[serde(rename = "Degraded")]
    Degraded = 2,
    /// Error
    #[serde(rename = "Error")]
    Error = 3,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 4,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 5,
}

impl Default for ManagedSystemElement_PrimaryStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

