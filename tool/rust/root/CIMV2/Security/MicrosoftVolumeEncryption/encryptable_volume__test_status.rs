// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source EncryptableVolume_TestStatus
//////////////////////////////////////////////

/// EncryptableVolume_TestStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum EncryptableVolume_TestStatus {
    /// NotFailed_and_NonePending
    #[serde(rename = "NotFailed_and_NonePending")]
    NotFailedAndNonePending = 0,
    /// Failed
    #[serde(rename = "Failed")]
    Failed = 1,
    /// Pending
    #[serde(rename = "Pending")]
    Pending = 2,
}

impl Default for EncryptableVolume_TestStatus {
    fn default() -> Self {
        Self::NotFailedAndNonePending
    }
}

