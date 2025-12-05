// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ElementSettingData_IsCurrent
//////////////////////////////////////////////

/// ElementSettingData_IsCurrent enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ElementSettingData_IsCurrent {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Is_Current
    #[serde(rename = "Is_Current")]
    IsCurrent = 1,
    /// Is_Not_Current
    #[serde(rename = "Is_Not_Current")]
    IsNotCurrent = 2,
}

impl Default for ElementSettingData_IsCurrent {
    fn default() -> Self {
        Self::Unknown
    }
}

