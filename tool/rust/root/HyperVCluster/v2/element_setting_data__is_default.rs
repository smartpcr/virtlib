// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ElementSettingData_IsDefault
//////////////////////////////////////////////

/// ElementSettingData_IsDefault enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ElementSettingData_IsDefault {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Is_Default
    #[serde(rename = "Is_Default")]
    IsDefault = 1,
    /// Is_Not_Default
    #[serde(rename = "Is_Not_Default")]
    IsNotDefault = 2,
}

impl Default for ElementSettingData_IsDefault {
    fn default() -> Self {
        Self::Unknown
    }
}

