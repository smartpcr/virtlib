// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ValidationDiskInfo_StackType
//////////////////////////////////////////////

/// ValidationDiskInfo_StackType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ValidationDiskInfo_StackType {
    /// _5
    #[serde(rename = "_5")]
    V5 = 0,
    /// _6
    #[serde(rename = "_6")]
    V6 = 1,
    /// _7
    #[serde(rename = "_7")]
    V7 = 2,
    /// _8
    #[serde(rename = "_8")]
    V8 = 3,
}

impl Default for ValidationDiskInfo_StackType {
    fn default() -> Self {
        Self::V5
    }
}

