// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageSettingData_ThreadCountPerChannel
//////////////////////////////////////////////

/// StorageSettingData_ThreadCountPerChannel enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageSettingData_ThreadCountPerChannel {
    /// Default
    #[serde(rename = "Default")]
    Default = 0,
    /// Low
    #[serde(rename = "Low")]
    Low = 1,
    /// Medium
    #[serde(rename = "Medium")]
    Medium = 2,
    /// High
    #[serde(rename = "High")]
    High = 3,
}

impl Default for StorageSettingData_ThreadCountPerChannel {
    fn default() -> Self {
        Self::Default
    }
}

