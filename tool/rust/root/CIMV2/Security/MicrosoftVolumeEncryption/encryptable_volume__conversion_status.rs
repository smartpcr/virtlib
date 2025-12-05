// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source EncryptableVolume_ConversionStatus
//////////////////////////////////////////////

/// EncryptableVolume_ConversionStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum EncryptableVolume_ConversionStatus {
    /// FullyDecrypted
    #[serde(rename = "FullyDecrypted")]
    FullyDecrypted = 0,
    /// FullyEncrypted
    #[serde(rename = "FullyEncrypted")]
    FullyEncrypted = 1,
    /// EncryptionInProgress
    #[serde(rename = "EncryptionInProgress")]
    EncryptionInProgress = 2,
    /// DecryptionInProgress
    #[serde(rename = "DecryptionInProgress")]
    DecryptionInProgress = 3,
    /// EncryptionPaused
    #[serde(rename = "EncryptionPaused")]
    EncryptionPaused = 4,
    /// DecryptionPaused
    #[serde(rename = "DecryptionPaused")]
    DecryptionPaused = 5,
}

impl Default for EncryptableVolume_ConversionStatus {
    fn default() -> Self {
        Self::FullyDecrypted
    }
}

