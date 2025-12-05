// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source AggregationMetricDefinition_SimpleFunction
//////////////////////////////////////////////

/// AggregationMetricDefinition_SimpleFunction enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum AggregationMetricDefinition_SimpleFunction {
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 1,
    /// Minimum
    #[serde(rename = "Minimum")]
    Minimum = 2,
    /// Maximum
    #[serde(rename = "Maximum")]
    Maximum = 3,
    /// Average
    #[serde(rename = "Average")]
    Average = 4,
    /// Median
    #[serde(rename = "Median")]
    Median = 5,
    /// Mode
    #[serde(rename = "Mode")]
    Mode = 6,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 7,
}

impl Default for AggregationMetricDefinition_SimpleFunction {
    fn default() -> Self {
        Self::DMTFReserved
    }
}

