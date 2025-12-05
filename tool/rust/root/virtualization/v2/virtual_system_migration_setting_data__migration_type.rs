// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemMigrationSettingData_MigrationType
//////////////////////////////////////////////

/// VirtualSystemMigrationSettingData_MigrationType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemMigrationSettingData_MigrationType {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Live
    #[serde(rename = "Live")]
    Live = 2,
    /// Resume
    #[serde(rename = "Resume")]
    Resume = 3,
    /// Restart
    #[serde(rename = "Restart")]
    Restart = 4,
}

impl Default for VirtualSystemMigrationSettingData_MigrationType {
    fn default() -> Self {
        Self::Unknown
    }
}

