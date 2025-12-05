// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source TaskSettings_Compatibility
//////////////////////////////////////////////

/// TaskSettings_Compatibility enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum TaskSettings_Compatibility {
    /// _8
    #[serde(rename = "_8")]
    V8 = 0,
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
}

impl Default for TaskSettings_Compatibility {
    fn default() -> Self {
        Self::V8
    }
}

