// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source EnabledLogicalElement_EnabledState
//////////////////////////////////////////////

/// EnabledLogicalElement_EnabledState enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum EnabledLogicalElement_EnabledState {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled = 2,
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled = 3,
    /// Shutting_Down
    #[serde(rename = "Shutting_Down")]
    ShuttingDown = 4,
    /// Not_Applicable
    #[serde(rename = "Not_Applicable")]
    NotApplicable = 5,
    /// Enabled_but_Offline
    #[serde(rename = "Enabled_but_Offline")]
    EnabledButOffline = 6,
    /// In_Test
    #[serde(rename = "In_Test")]
    InTest = 7,
    /// Deferred
    #[serde(rename = "Deferred")]
    Deferred = 8,
    /// Quiesce
    #[serde(rename = "Quiesce")]
    Quiesce = 9,
    /// Starting
    #[serde(rename = "Starting")]
    Starting = 10,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 11,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 12,
}

impl Default for EnabledLogicalElement_EnabledState {
    fn default() -> Self {
        Self::Unknown
    }
}

