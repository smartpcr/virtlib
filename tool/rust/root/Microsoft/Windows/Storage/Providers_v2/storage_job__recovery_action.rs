// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageJob_RecoveryAction
//////////////////////////////////////////////

/// StorageJob_RecoveryAction enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageJob_RecoveryAction {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Do_Not_Continue
    #[serde(rename = "Do_Not_Continue")]
    DoNotContinue = 2,
    /// Continue_With_Next_Job
    #[serde(rename = "Continue_With_Next_Job")]
    ContinueWithNextJob = 3,
    /// Re_run_Job
    #[serde(rename = "Re_run_Job")]
    ReRunJob = 4,
}

impl Default for StorageJob_RecoveryAction {
    fn default() -> Self {
        Self::Unknown
    }
}

