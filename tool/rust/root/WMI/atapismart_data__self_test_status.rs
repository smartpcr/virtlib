// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ATAPISmartData_SelfTestStatus
//////////////////////////////////////////////

/// ATAPISmartData_SelfTestStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ATAPISmartData_SelfTestStatus {
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
    /// _5
    #[serde(rename = "_5")]
    V5 = 5,
    /// _6
    #[serde(rename = "_6")]
    V6 = 6,
    /// _7
    #[serde(rename = "_7")]
    V7 = 7,
    /// _15
    #[serde(rename = "_15")]
    V15 = 8,
}

impl Default for ATAPISmartData_SelfTestStatus {
    fn default() -> Self {
        Self::V0
    }
}

