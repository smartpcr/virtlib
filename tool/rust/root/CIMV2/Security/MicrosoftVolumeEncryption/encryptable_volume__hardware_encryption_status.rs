// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source EncryptableVolume_HardwareEncryptionStatus
//////////////////////////////////////////////

/// EncryptableVolume_HardwareEncryptionStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum EncryptableVolume_HardwareEncryptionStatus {
    /// Not_supported
    #[serde(rename = "Not_supported")]
    NotSupported = 0,
    /// No_protection
    #[serde(rename = "No_protection")]
    NoProtection = 1,
    /// Uses_software
    #[serde(rename = "Uses_software")]
    UsesSoftware = 2,
    /// Uses_hardware
    #[serde(rename = "Uses_hardware")]
    UsesHardware = 3,
}

impl Default for EncryptableVolume_HardwareEncryptionStatus {
    fn default() -> Self {
        Self::NotSupported
    }
}

