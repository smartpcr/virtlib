// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source EncryptableVolume_LockStatus
//////////////////////////////////////////////

/// EncryptableVolume_LockStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum EncryptableVolume_LockStatus {
    /// Unlocked
    #[serde(rename = "Unlocked")]
    Unlocked = 0,
    /// Locked
    #[serde(rename = "Locked")]
    Locked = 1,
}

impl Default for EncryptableVolume_LockStatus {
    fn default() -> Self {
        Self::Unlocked
    }
}

