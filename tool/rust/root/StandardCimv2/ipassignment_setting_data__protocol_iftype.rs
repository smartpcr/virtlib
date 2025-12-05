// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source IPAssignmentSettingData_ProtocolIFType
//////////////////////////////////////////////

/// IPAssignmentSettingData_ProtocolIFType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum IPAssignmentSettingData_ProtocolIFType {
    /// _10
    #[serde(rename = "_10")]
    V10 = 0,
    /// _29
    #[serde(rename = "_29")]
    V29 = 1,
    /// _383
    #[serde(rename = "_383")]
    V383 = 4096,
    /// _384
    #[serde(rename = "_384")]
    V384 = 4097,
    /// _30
    #[serde(rename = "_30")]
    V30 = 4098,
}

impl Default for IPAssignmentSettingData_ProtocolIFType {
    fn default() -> Self {
        Self::V10
    }
}

