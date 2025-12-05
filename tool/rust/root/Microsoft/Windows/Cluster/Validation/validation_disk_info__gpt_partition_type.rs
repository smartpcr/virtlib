// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ValidationDiskInfo_GptPartitionType
//////////////////////////////////////////////

/// ValidationDiskInfo_GptPartitionType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ValidationDiskInfo_GptPartitionType {
    /// _28
    #[serde(rename = "_28")]
    V28 = 0,
    /// _29
    #[serde(rename = "_29")]
    V29 = 1,
    /// _30
    #[serde(rename = "_30")]
    V30 = 2,
    /// _31
    #[serde(rename = "_31")]
    V31 = 3,
    /// _32
    #[serde(rename = "_32")]
    V32 = 4,
    /// _33
    #[serde(rename = "_33")]
    V33 = 5,
    /// _34
    #[serde(rename = "_34")]
    V34 = 6,
    /// _35
    #[serde(rename = "_35")]
    V35 = 7,
    /// _36
    #[serde(rename = "_36")]
    V36 = 8,
    /// _24
    #[serde(rename = "_24")]
    V24 = 9,
    /// _37
    #[serde(rename = "_37")]
    V37 = 10,
    /// _38
    #[serde(rename = "_38")]
    V38 = 11,
    /// _39
    #[serde(rename = "_39")]
    V39 = 12,
    /// _40
    #[serde(rename = "_40")]
    V40 = 13,
    /// _5
    #[serde(rename = "_5")]
    V5 = 14,
}

impl Default for ValidationDiskInfo_GptPartitionType {
    fn default() -> Self {
        Self::V28
    }
}

