// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source BaseMetricDefinition_GatheringType
//////////////////////////////////////////////

/// BaseMetricDefinition_GatheringType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum BaseMetricDefinition_GatheringType {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// OnChange
    #[serde(rename = "OnChange")]
    OnChange = 2,
    /// Periodic
    #[serde(rename = "Periodic")]
    Periodic = 3,
    /// OnRequest
    #[serde(rename = "OnRequest")]
    OnRequest = 4,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 5,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 6,
}

impl Default for BaseMetricDefinition_GatheringType {
    fn default() -> Self {
        Self::Unknown
    }
}

