// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ValidationDiskInfo_BusType
//////////////////////////////////////////////

/// ValidationDiskInfo_BusType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ValidationDiskInfo_BusType {
    /// _5
    #[serde(rename = "_5")]
    V5 = 0,
    /// _9
    #[serde(rename = "_9")]
    V9 = 1,
    /// _10
    #[serde(rename = "_10")]
    V10 = 2,
    /// _11
    #[serde(rename = "_11")]
    V11 = 3,
    /// _12
    #[serde(rename = "_12")]
    V12 = 4,
    /// _13
    #[serde(rename = "_13")]
    V13 = 5,
    /// _14
    #[serde(rename = "_14")]
    V14 = 6,
    /// _15
    #[serde(rename = "_15")]
    V15 = 7,
    /// _16
    #[serde(rename = "_16")]
    V16 = 8,
    /// _17
    #[serde(rename = "_17")]
    V17 = 9,
    /// _18
    #[serde(rename = "_18")]
    V18 = 10,
    /// _19
    #[serde(rename = "_19")]
    V19 = 11,
    /// _20
    #[serde(rename = "_20")]
    V20 = 12,
    /// _21
    #[serde(rename = "_21")]
    V21 = 13,
    /// _22
    #[serde(rename = "_22")]
    V22 = 14,
    /// _23
    #[serde(rename = "_23")]
    V23 = 15,
    /// _24
    #[serde(rename = "_24")]
    V24 = 16,
    /// _25
    #[serde(rename = "_25")]
    V25 = 17,
    /// _26
    #[serde(rename = "_26")]
    V26 = 18,
    /// _27
    #[serde(rename = "_27")]
    V27 = 19,
}

impl Default for ValidationDiskInfo_BusType {
    fn default() -> Self {
        Self::V5
    }
}

