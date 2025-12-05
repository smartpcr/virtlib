// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemSnapshotSettingData_ConsistencyLevel
//////////////////////////////////////////////

/// VirtualSystemSnapshotSettingData_ConsistencyLevel enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemSnapshotSettingData_ConsistencyLevel {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Application_Consistent
    #[serde(rename = "Application_Consistent")]
    ApplicationConsistent = 1,
    /// Crash_Consistent
    #[serde(rename = "Crash_Consistent")]
    CrashConsistent = 2,
}

impl Default for VirtualSystemSnapshotSettingData_ConsistencyLevel {
    fn default() -> Self {
        Self::Unknown
    }
}

