// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Synchronized_CopyPriority
//////////////////////////////////////////////

/// Synchronized_CopyPriority enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Synchronized_CopyPriority {
    /// Not_Managed
    #[serde(rename = "Not_Managed")]
    NotManaged = 0,
    /// Low
    #[serde(rename = "Low")]
    Low = 1,
    /// Same
    #[serde(rename = "Same")]
    Same = 2,
    /// High
    #[serde(rename = "High")]
    High = 3,
    /// Microsoft_Reserved
    #[serde(rename = "Microsoft_Reserved")]
    MicrosoftReserved = 4,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 5,
}

impl Default for Synchronized_CopyPriority {
    fn default() -> Self {
        Self::NotManaged
    }
}

