// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageDiagnoseResult_PerceivedSeverity
//////////////////////////////////////////////

/// StorageDiagnoseResult_PerceivedSeverity enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageDiagnoseResult_PerceivedSeverity {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Information
    #[serde(rename = "Information")]
    Information = 2,
    /// Degraded_Warning
    #[serde(rename = "Degraded_Warning")]
    DegradedWarning = 3,
    /// Minor
    #[serde(rename = "Minor")]
    Minor = 4,
    /// Major
    #[serde(rename = "Major")]
    Major = 5,
    /// Critical
    #[serde(rename = "Critical")]
    Critical = 6,
    /// Fatal_NonRecoverable
    #[serde(rename = "Fatal_NonRecoverable")]
    FatalNonRecoverable = 7,
}

impl Default for StorageDiagnoseResult_PerceivedSeverity {
    fn default() -> Self {
        Self::Unknown
    }
}

