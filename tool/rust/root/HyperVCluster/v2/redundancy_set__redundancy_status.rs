// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source RedundancySet_RedundancyStatus
//////////////////////////////////////////////

/// RedundancySet_RedundancyStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum RedundancySet_RedundancyStatus {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 1,
    /// Fully_Redundant
    #[serde(rename = "Fully_Redundant")]
    FullyRedundant = 2,
    /// Degraded_Redundancy
    #[serde(rename = "Degraded_Redundancy")]
    DegradedRedundancy = 3,
    /// Redundancy_Lost
    #[serde(rename = "Redundancy_Lost")]
    RedundancyLost = 4,
    /// Overall_Failure
    #[serde(rename = "Overall_Failure")]
    OverallFailure = 5,
}

impl Default for RedundancySet_RedundancyStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

