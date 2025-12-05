// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MetricService_Range
//////////////////////////////////////////////

/// MetricService_Range enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MetricService_Range {
    /// Minimum
    #[serde(rename = "Minimum")]
    Minimum = 2,
    /// Maximum
    #[serde(rename = "Maximum")]
    Maximum = 3,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 4,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 5,
}

impl Default for MetricService_Range {
    fn default() -> Self {
        Self::Minimum
    }
}

