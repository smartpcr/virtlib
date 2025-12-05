// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source EnabledLogicalElement_RequestedState
//////////////////////////////////////////////

/// EnabledLogicalElement_RequestedState enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum EnabledLogicalElement_RequestedState {
    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled = 2,
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled = 3,
    /// Shut_Down
    #[serde(rename = "Shut_Down")]
    ShutDown = 4,
    /// Offline
    #[serde(rename = "Offline")]
    Offline = 6,
    /// Test
    #[serde(rename = "Test")]
    Test = 7,
    /// Defer
    #[serde(rename = "Defer")]
    Defer = 8,
    /// Quiesce
    #[serde(rename = "Quiesce")]
    Quiesce = 9,
    /// Reboot
    #[serde(rename = "Reboot")]
    Reboot = 10,
    /// Reset
    #[serde(rename = "Reset")]
    Reset = 11,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 12,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 13,
}

impl Default for EnabledLogicalElement_RequestedState {
    fn default() -> Self {
        Self::Enabled
    }
}

