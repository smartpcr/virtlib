// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemMigrationSettingData_CPUCappingMagnitude
//////////////////////////////////////////////

/// VirtualSystemMigrationSettingData_CPUCappingMagnitude enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemMigrationSettingData_CPUCappingMagnitude {
    /// Normal
    #[serde(rename = "Normal")]
    Normal = 0,
    /// Low
    #[serde(rename = "Low")]
    Low = 1,
    /// High
    #[serde(rename = "High")]
    High = 2,
}

impl Default for VirtualSystemMigrationSettingData_CPUCappingMagnitude {
    fn default() -> Self {
        Self::Normal
    }
}

