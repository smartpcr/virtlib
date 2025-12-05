// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source TargetLoginOptions_HeaderDigest
//////////////////////////////////////////////

/// TargetLoginOptions_HeaderDigest enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum TargetLoginOptions_HeaderDigest {
    /// None
    #[serde(rename = "None")]
    None = 0,
    /// CRC32C
    #[serde(rename = "CRC32C")]
    CRC32C = 1,
}

impl Default for TargetLoginOptions_HeaderDigest {
    fn default() -> Self {
        Self::None
    }
}

