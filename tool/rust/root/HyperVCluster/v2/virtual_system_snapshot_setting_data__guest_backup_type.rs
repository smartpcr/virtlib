// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemSnapshotSettingData_GuestBackupType
//////////////////////////////////////////////

/// VirtualSystemSnapshotSettingData_GuestBackupType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemSnapshotSettingData_GuestBackupType {
    /// Undefined
    #[serde(rename = "Undefined")]
    Undefined = 0,
    /// Full
    #[serde(rename = "Full")]
    Full = 1,
    /// Copy
    #[serde(rename = "Copy")]
    Copy = 2,
}

impl Default for VirtualSystemSnapshotSettingData_GuestBackupType {
    fn default() -> Self {
        Self::Undefined
    }
}

