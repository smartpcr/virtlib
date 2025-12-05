// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SmbBandwidthLimit_Category
//////////////////////////////////////////////

/// SmbBandwidthLimit_Category enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SmbBandwidthLimit_Category {
    /// _9
    #[serde(rename = "_9")]
    V9 = 0,
    /// _53
    #[serde(rename = "_53")]
    V53 = 1,
    /// _54
    #[serde(rename = "_54")]
    V54 = 2,
}

impl Default for SmbBandwidthLimit_Category {
    fn default() -> Self {
        Self::V9
    }
}

