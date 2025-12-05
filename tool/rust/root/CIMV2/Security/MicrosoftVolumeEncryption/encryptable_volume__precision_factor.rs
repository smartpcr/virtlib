// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source EncryptableVolume_PrecisionFactor
//////////////////////////////////////////////

/// EncryptableVolume_PrecisionFactor enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum EncryptableVolume_PrecisionFactor {
    /// _0
    #[serde(rename = "_0")]
    V0 = 0,
    /// _1
    #[serde(rename = "_1")]
    V1 = 1,
    /// _2
    #[serde(rename = "_2")]
    V2 = 2,
    /// _3
    #[serde(rename = "_3")]
    V3 = 3,
    /// _4
    #[serde(rename = "_4")]
    V4 = 4,
}

impl Default for EncryptableVolume_PrecisionFactor {
    fn default() -> Self {
        Self::V0
    }
}

