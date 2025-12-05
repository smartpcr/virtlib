// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ValidationDiskInfo_MediaType
//////////////////////////////////////////////

/// ValidationDiskInfo_MediaType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ValidationDiskInfo_MediaType {
    /// _5
    #[serde(rename = "_5")]
    V5 = 0,
    /// _92
    #[serde(rename = "_92")]
    V92 = 1,
    /// _93
    #[serde(rename = "_93")]
    V93 = 2,
    /// _26
    #[serde(rename = "_26")]
    V26 = 3,
}

impl Default for ValidationDiskInfo_MediaType {
    fn default() -> Self {
        Self::V5
    }
}

