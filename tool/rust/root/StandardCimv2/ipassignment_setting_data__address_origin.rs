// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source IPAssignmentSettingData_AddressOrigin
//////////////////////////////////////////////

/// IPAssignmentSettingData_AddressOrigin enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum IPAssignmentSettingData_AddressOrigin {
    /// _10
    #[serde(rename = "_10")]
    V10 = 0,
    /// _11
    #[serde(rename = "_11")]
    V11 = 1,
    /// _349
    #[serde(rename = "_349")]
    V349 = 2,
    /// _690
    #[serde(rename = "_690")]
    V690 = 3,
    /// _691
    #[serde(rename = "_691")]
    V691 = 4,
    /// _692
    #[serde(rename = "_692")]
    V692 = 5,
    /// _29
    #[serde(rename = "_29")]
    V29 = 6,
    /// _30
    #[serde(rename = "_30")]
    V30 = 7,
}

impl Default for IPAssignmentSettingData_AddressOrigin {
    fn default() -> Self {
        Self::V10
    }
}

