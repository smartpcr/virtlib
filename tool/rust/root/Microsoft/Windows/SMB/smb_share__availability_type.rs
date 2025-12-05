// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SmbShare_AvailabilityType
//////////////////////////////////////////////

/// SmbShare_AvailabilityType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SmbShare_AvailabilityType {
    /// _26
    #[serde(rename = "_26")]
    V26 = 0,
    /// _27
    #[serde(rename = "_27")]
    V27 = 1,
    /// _28
    #[serde(rename = "_28")]
    V28 = 2,
    /// _10
    #[serde(rename = "_10")]
    V10 = 3,
    /// _29
    #[serde(rename = "_29")]
    V29 = 4,
}

impl Default for SmbShare_AvailabilityType {
    fn default() -> Self {
        Self::V26
    }
}

