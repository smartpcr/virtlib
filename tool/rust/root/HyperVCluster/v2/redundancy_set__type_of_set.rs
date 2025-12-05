// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source RedundancySet_TypeOfSet
//////////////////////////////////////////////

/// RedundancySet_TypeOfSet enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum RedundancySet_TypeOfSet {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Nplus1
    #[serde(rename = "Nplus1")]
    Nplus1 = 2,
    /// Load_Balanced
    #[serde(rename = "Load_Balanced")]
    LoadBalanced = 3,
    /// Sparing
    #[serde(rename = "Sparing")]
    Sparing = 4,
    /// Limited_Sparing
    #[serde(rename = "Limited_Sparing")]
    LimitedSparing = 5,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 6,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 7,
}

impl Default for RedundancySet_TypeOfSet {
    fn default() -> Self {
        Self::Unknown
    }
}

