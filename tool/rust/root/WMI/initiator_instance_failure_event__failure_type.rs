// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source InitiatorInstanceFailureEvent_FailureType
//////////////////////////////////////////////

/// InitiatorInstanceFailureEvent_FailureType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum InitiatorInstanceFailureEvent_FailureType {
    /// SessionDigestError
    #[serde(rename = "SessionDigestError")]
    SessionDigestError = 0,
    /// SessionCxnTimeoutError
    #[serde(rename = "SessionCxnTimeoutError")]
    SessionCxnTimeoutError = 1,
    /// SessionFormatError
    #[serde(rename = "SessionFormatError")]
    SessionFormatError = 2,
}

impl Default for InitiatorInstanceFailureEvent_FailureType {
    fn default() -> Self {
        Self::SessionDigestError
    }
}

