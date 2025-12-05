// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ManagedSystemElement_HealthState
//////////////////////////////////////////////

/// ManagedSystemElement_HealthState enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ManagedSystemElement_HealthState {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// OK
    #[serde(rename = "OK")]
    OK = 5,
    /// Degraded_Warning
    #[serde(rename = "Degraded_Warning")]
    DegradedWarning = 10,
    /// Minor_failure
    #[serde(rename = "Minor_failure")]
    MinorFailure = 15,
    /// Major_failure
    #[serde(rename = "Major_failure")]
    MajorFailure = 20,
    /// Critical_failure
    #[serde(rename = "Critical_failure")]
    CriticalFailure = 25,
    /// Non_recoverable_error
    #[serde(rename = "Non_recoverable_error")]
    NonRecoverableError = 30,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 31,
}

impl Default for ManagedSystemElement_HealthState {
    fn default() -> Self {
        Self::Unknown
    }
}

