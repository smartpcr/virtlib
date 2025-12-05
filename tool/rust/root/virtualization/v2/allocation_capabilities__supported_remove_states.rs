// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source AllocationCapabilities_SupportedRemoveStates
//////////////////////////////////////////////

/// AllocationCapabilities_SupportedRemoveStates enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum AllocationCapabilities_SupportedRemoveStates {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
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
    /// Paused
    #[serde(rename = "Paused")]
    Paused = 11,
    /// Suspended
    #[serde(rename = "Suspended")]
    Suspended = 12,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 13,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 14,
}

impl Default for AllocationCapabilities_SupportedRemoveStates {
    fn default() -> Self {
        Self::Unknown
    }
}

