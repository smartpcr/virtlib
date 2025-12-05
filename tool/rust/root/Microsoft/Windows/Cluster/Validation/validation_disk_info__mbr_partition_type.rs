// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ValidationDiskInfo_MbrPartitionType
//////////////////////////////////////////////

/// ValidationDiskInfo_MbrPartitionType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ValidationDiskInfo_MbrPartitionType {
    /// _28
    #[serde(rename = "_28")]
    V28 = 0,
    /// _41
    #[serde(rename = "_41")]
    V41 = 1,
    /// _42
    #[serde(rename = "_42")]
    V42 = 2,
    /// _43
    #[serde(rename = "_43")]
    V43 = 3,
    /// _44
    #[serde(rename = "_44")]
    V44 = 4,
    /// _45
    #[serde(rename = "_45")]
    V45 = 5,
    /// _46
    #[serde(rename = "_46")]
    V46 = 6,
    /// _47
    #[serde(rename = "_47")]
    V47 = 7,
    /// _48
    #[serde(rename = "_48")]
    V48 = 8,
    /// _49
    #[serde(rename = "_49")]
    V49 = 9,
    /// _50
    #[serde(rename = "_50")]
    V50 = 10,
    /// _51
    #[serde(rename = "_51")]
    V51 = 11,
    /// _52
    #[serde(rename = "_52")]
    V52 = 12,
    /// _53
    #[serde(rename = "_53")]
    V53 = 13,
    /// _54
    #[serde(rename = "_54")]
    V54 = 14,
    /// _24
    #[serde(rename = "_24")]
    V24 = 15,
    /// _5
    #[serde(rename = "_5")]
    V5 = 16,
}

impl Default for ValidationDiskInfo_MbrPartitionType {
    fn default() -> Self {
        Self::V28
    }
}

