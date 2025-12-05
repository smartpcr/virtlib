// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ATAPISmartData_OfflineCollectionStatus
//////////////////////////////////////////////

/// ATAPISmartData_OfflineCollectionStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ATAPISmartData_OfflineCollectionStatus {
    /// _0
    #[serde(rename = "_0")]
    V0 = 0,
    /// _2
    #[serde(rename = "_2")]
    V2 = 1,
    /// _4
    #[serde(rename = "_4")]
    V4 = 2,
    /// _5
    #[serde(rename = "_5")]
    V5 = 3,
    /// _6
    #[serde(rename = "_6")]
    V6 = 4,
    /// _128
    #[serde(rename = "_128")]
    V128 = 5,
    /// _130
    #[serde(rename = "_130")]
    V130 = 6,
    /// _132
    #[serde(rename = "_132")]
    V132 = 7,
    /// _133
    #[serde(rename = "_133")]
    V133 = 8,
    /// _134
    #[serde(rename = "_134")]
    V134 = 9,
}

impl Default for ATAPISmartData_OfflineCollectionStatus {
    fn default() -> Self {
        Self::V0
    }
}

