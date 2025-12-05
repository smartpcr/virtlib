// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source InitiatorNodeFailureEvent_FailureType
//////////////////////////////////////////////

/// InitiatorNodeFailureEvent_FailureType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum InitiatorNodeFailureEvent_FailureType {
    /// LoginOtherFail
    #[serde(rename = "LoginOtherFail")]
    LoginOtherFail = 0,
    /// LoginAuthFail
    #[serde(rename = "LoginAuthFail")]
    LoginAuthFail = 1,
    /// LoginAuthenticateFail
    #[serde(rename = "LoginAuthenticateFail")]
    LoginAuthenticateFail = 2,
    /// LoginNegotiateFail
    #[serde(rename = "LoginNegotiateFail")]
    LoginNegotiateFail = 3,
    /// LogoutOthers
    #[serde(rename = "LogoutOthers")]
    LogoutOthers = 4,
}

impl Default for InitiatorNodeFailureEvent_FailureType {
    fn default() -> Self {
        Self::LoginOtherFail
    }
}

