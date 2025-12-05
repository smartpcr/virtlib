// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SmbShare_LeasingMode
//////////////////////////////////////////////

/// SmbShare_LeasingMode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SmbShare_LeasingMode {
    /// _3
    #[serde(rename = "_3")]
    V3 = 0,
    /// _58
    #[serde(rename = "_58")]
    V58 = 1,
    /// _18
    #[serde(rename = "_18")]
    V18 = 2,
}

impl Default for SmbShare_LeasingMode {
    fn default() -> Self {
        Self::V3
    }
}

