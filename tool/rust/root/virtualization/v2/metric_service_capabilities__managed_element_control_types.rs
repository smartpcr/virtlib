// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MetricServiceCapabilities_ManagedElementControlTypes
//////////////////////////////////////////////

/// MetricServiceCapabilities_ManagedElementControlTypes enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MetricServiceCapabilities_ManagedElementControlTypes {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Discrete
    #[serde(rename = "Discrete")]
    Discrete = 2,
    /// Bulk
    #[serde(rename = "Bulk")]
    Bulk = 3,
    /// Both
    #[serde(rename = "Both")]
    Both = 4,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 5,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 6,
}

impl Default for MetricServiceCapabilities_ManagedElementControlTypes {
    fn default() -> Self {
        Self::Unknown
    }
}

