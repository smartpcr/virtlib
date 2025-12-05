// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source BaseMetricDefinition_ChangeType
//////////////////////////////////////////////

/// BaseMetricDefinition_ChangeType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum BaseMetricDefinition_ChangeType {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// N_A
    #[serde(rename = "N_A")]
    NA = 2,
    /// Counter
    #[serde(rename = "Counter")]
    Counter = 3,
    /// Gauge
    #[serde(rename = "Gauge")]
    Gauge = 4,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 5,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 6,
}

impl Default for BaseMetricDefinition_ChangeType {
    fn default() -> Self {
        Self::Unknown
    }
}

