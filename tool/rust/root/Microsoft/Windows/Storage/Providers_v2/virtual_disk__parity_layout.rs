// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualDisk_ParityLayout
//////////////////////////////////////////////

/// VirtualDisk_ParityLayout enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualDisk_ParityLayout {
    /// Non_rotated_Parity
    #[serde(rename = "Non_rotated_Parity")]
    NonRotatedParity = 1,
    /// Rotated_Parity
    #[serde(rename = "Rotated_Parity")]
    RotatedParity = 2,
}

impl Default for VirtualDisk_ParityLayout {
    fn default() -> Self {
        Self::NonRotatedParity
    }
}

