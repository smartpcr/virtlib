// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source BaseMetricDefinition_TimeScope
//////////////////////////////////////////////

/// BaseMetricDefinition_TimeScope enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum BaseMetricDefinition_TimeScope {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Point
    #[serde(rename = "Point")]
    Point = 2,
    /// Interval
    #[serde(rename = "Interval")]
    Interval = 3,
    /// StartupInterval
    #[serde(rename = "StartupInterval")]
    StartupInterval = 4,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 5,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 6,
}

impl Default for BaseMetricDefinition_TimeScope {
    fn default() -> Self {
        Self::Unknown
    }
}

