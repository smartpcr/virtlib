// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source WinSAT_WinSATAssessmentState
//////////////////////////////////////////////

/// WinSAT_WinSATAssessmentState enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum WinSAT_WinSATAssessmentState {
    /// StateUnknown
    #[serde(rename = "StateUnknown")]
    StateUnknown = 0,
    /// Valid
    #[serde(rename = "Valid")]
    Valid = 1,
    /// IncoherentWithHardware
    #[serde(rename = "IncoherentWithHardware")]
    IncoherentWithHardware = 2,
    /// NoAssessmentAvailable
    #[serde(rename = "NoAssessmentAvailable")]
    NoAssessmentAvailable = 3,
    /// Invalid
    #[serde(rename = "Invalid")]
    Invalid = 4,
}

impl Default for WinSAT_WinSATAssessmentState {
    fn default() -> Self {
        Self::StateUnknown
    }
}

