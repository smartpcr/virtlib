// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ElementCapabilities_Characteristics
//////////////////////////////////////////////

/// ElementCapabilities_Characteristics enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ElementCapabilities_Characteristics {
    /// Default
    #[serde(rename = "Default")]
    Default = 2,
    /// Current
    #[serde(rename = "Current")]
    Current = 3,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 4,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 5,
}

impl Default for ElementCapabilities_Characteristics {
    fn default() -> Self {
        Self::Default
    }
}

