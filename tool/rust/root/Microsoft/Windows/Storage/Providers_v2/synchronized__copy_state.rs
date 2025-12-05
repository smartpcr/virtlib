// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Synchronized_CopyState
//////////////////////////////////////////////

/// Synchronized_CopyState enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Synchronized_CopyState {
    /// Initialized
    #[serde(rename = "Initialized")]
    Initialized = 2,
    /// Unsynchronized
    #[serde(rename = "Unsynchronized")]
    Unsynchronized = 3,
    /// Synchronized
    #[serde(rename = "Synchronized")]
    Synchronized = 4,
    /// Broken
    #[serde(rename = "Broken")]
    Broken = 5,
    /// Fractured
    #[serde(rename = "Fractured")]
    Fractured = 6,
    /// Split
    #[serde(rename = "Split")]
    Split = 7,
    /// Inactive
    #[serde(rename = "Inactive")]
    Inactive = 8,
    /// Suspended
    #[serde(rename = "Suspended")]
    Suspended = 9,
    /// Failedover
    #[serde(rename = "Failedover")]
    Failedover = 10,
    /// Prepared
    #[serde(rename = "Prepared")]
    Prepared = 11,
    /// Aborted
    #[serde(rename = "Aborted")]
    Aborted = 12,
    /// Skewed
    #[serde(rename = "Skewed")]
    Skewed = 13,
    /// Mixed
    #[serde(rename = "Mixed")]
    Mixed = 14,
    /// Not_Applicable
    #[serde(rename = "Not_Applicable")]
    NotApplicable = 15,
    /// Microsoft_Reserved
    #[serde(rename = "Microsoft_Reserved")]
    MicrosoftReserved = 16,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 17,
}

impl Default for Synchronized_CopyState {
    fn default() -> Self {
        Self::Initialized
    }
}

