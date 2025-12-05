// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SmbShare_CachingMode
//////////////////////////////////////////////

/// SmbShare_CachingMode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SmbShare_CachingMode {
    /// _18
    #[serde(rename = "_18")]
    V18 = 0,
    /// _19
    #[serde(rename = "_19")]
    V19 = 1,
    /// _20
    #[serde(rename = "_20")]
    V20 = 2,
    /// _21
    #[serde(rename = "_21")]
    V21 = 3,
    /// _22
    #[serde(rename = "_22")]
    V22 = 4,
    /// _15
    #[serde(rename = "_15")]
    V15 = 5,
}

impl Default for SmbShare_CachingMode {
    fn default() -> Self {
        Self::V18
    }
}

