// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ReferencePointCollection_ReferencePointType
//////////////////////////////////////////////

/// ReferencePointCollection_ReferencePointType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ReferencePointCollection_ReferencePointType {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Log_based
    #[serde(rename = "Log_based")]
    LogBased = 1,
    /// RCT_based
    #[serde(rename = "RCT_based")]
    RCTBased = 2,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 3,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 4,
}

impl Default for ReferencePointCollection_ReferencePointType {
    fn default() -> Self {
        Self::Unknown
    }
}

