// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MetricService_MetricCollectionEnabled
//////////////////////////////////////////////

/// MetricService_MetricCollectionEnabled enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MetricService_MetricCollectionEnabled {
    /// Enable
    #[serde(rename = "Enable")]
    Enable = 2,
    /// Disable
    #[serde(rename = "Disable")]
    Disable = 3,
    /// Reserved
    #[serde(rename = "Reserved")]
    Reserved = 4,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 5,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 6,
}

impl Default for MetricService_MetricCollectionEnabled {
    fn default() -> Self {
        Self::Enable
    }
}

