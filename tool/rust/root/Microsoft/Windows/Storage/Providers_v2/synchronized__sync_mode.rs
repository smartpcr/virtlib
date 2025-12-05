// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Synchronized_SyncMode
//////////////////////////////////////////////

/// Synchronized_SyncMode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Synchronized_SyncMode {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Synchronous
    #[serde(rename = "Synchronous")]
    Synchronous = 2,
    /// Asynchronous
    #[serde(rename = "Asynchronous")]
    Asynchronous = 3,
    /// Microsoft_Reserved
    #[serde(rename = "Microsoft_Reserved")]
    MicrosoftReserved = 4,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 5,
}

impl Default for Synchronized_SyncMode {
    fn default() -> Self {
        Self::Unknown
    }
}

