// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source WiFiEndpoint_BSSType
//////////////////////////////////////////////

/// WiFiEndpoint_BSSType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum WiFiEndpoint_BSSType {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Independent
    #[serde(rename = "Independent")]
    Independent = 2,
    /// Infrastructure
    #[serde(rename = "Infrastructure")]
    Infrastructure = 3,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 4,
}

impl Default for WiFiEndpoint_BSSType {
    fn default() -> Self {
        Self::Unknown
    }
}

