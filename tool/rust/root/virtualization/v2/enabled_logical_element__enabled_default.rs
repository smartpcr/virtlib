// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source EnabledLogicalElement_EnabledDefault
//////////////////////////////////////////////

/// EnabledLogicalElement_EnabledDefault enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum EnabledLogicalElement_EnabledDefault {
    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled = 2,
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled = 3,
    /// Not_Applicable
    #[serde(rename = "Not_Applicable")]
    NotApplicable = 5,
    /// Enabled_but_Offline
    #[serde(rename = "Enabled_but_Offline")]
    EnabledButOffline = 6,
    /// No_Default
    #[serde(rename = "No_Default")]
    NoDefault = 7,
    /// Quiesce
    #[serde(rename = "Quiesce")]
    Quiesce = 9,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 10,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 11,
}

impl Default for EnabledLogicalElement_EnabledDefault {
    fn default() -> Self {
        Self::Enabled
    }
}

