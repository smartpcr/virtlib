// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ElementSettingData_IsNext
//////////////////////////////////////////////

/// ElementSettingData_IsNext enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ElementSettingData_IsNext {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Is_Next
    #[serde(rename = "Is_Next")]
    IsNext = 1,
    /// Is_Not_Next
    #[serde(rename = "Is_Not_Next")]
    IsNotNext = 2,
    /// Is_Next_For_Single_Use
    #[serde(rename = "Is_Next_For_Single_Use")]
    IsNextForSingleUse = 3,
}

impl Default for ElementSettingData_IsNext {
    fn default() -> Self {
        Self::Unknown
    }
}

