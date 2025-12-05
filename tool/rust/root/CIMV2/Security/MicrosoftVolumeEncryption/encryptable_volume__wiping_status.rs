// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source EncryptableVolume_WipingStatus
//////////////////////////////////////////////

/// EncryptableVolume_WipingStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum EncryptableVolume_WipingStatus {
    /// FreeSpaceNotWiped
    #[serde(rename = "FreeSpaceNotWiped")]
    FreeSpaceNotWiped = 0,
    /// FreeSpaceWiped
    #[serde(rename = "FreeSpaceWiped")]
    FreeSpaceWiped = 1,
    /// FreeSpaceWipingInProgress
    #[serde(rename = "FreeSpaceWipingInProgress")]
    FreeSpaceWipingInProgress = 2,
    /// FreeSpaceWipingPaused
    #[serde(rename = "FreeSpaceWipingPaused")]
    FreeSpaceWipingPaused = 3,
}

impl Default for EncryptableVolume_WipingStatus {
    fn default() -> Self {
        Self::FreeSpaceNotWiped
    }
}

