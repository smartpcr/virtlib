// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MetricServiceCapabilities_SupportedMethods
//////////////////////////////////////////////

/// MetricServiceCapabilities_SupportedMethods enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MetricServiceCapabilities_SupportedMethods {
    /// ControlMetrics
    #[serde(rename = "ControlMetrics")]
    ControlMetrics = 2,
    /// ControlMetricsByClass
    #[serde(rename = "ControlMetricsByClass")]
    ControlMetricsByClass = 3,
    /// ShowMetrics
    #[serde(rename = "ShowMetrics")]
    ShowMetrics = 4,
    /// ShowMetricsByClass
    #[serde(rename = "ShowMetricsByClass")]
    ShowMetricsByClass = 5,
    /// GetMetricValues
    #[serde(rename = "GetMetricValues")]
    GetMetricValues = 6,
    /// ControlSampleTimes
    #[serde(rename = "ControlSampleTimes")]
    ControlSampleTimes = 7,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 8,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 9,
}

impl Default for MetricServiceCapabilities_SupportedMethods {
    fn default() -> Self {
        Self::ControlMetrics
    }
}

