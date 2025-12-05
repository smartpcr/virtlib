// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ConnectionStaticInfo_State
//////////////////////////////////////////////

/// ConnectionStaticInfo_State enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ConnectionStaticInfo_State {
    /// login
    #[serde(rename = "login")]
    Login = 0,
    /// full
    #[serde(rename = "full")]
    Full = 1,
    /// logout
    #[serde(rename = "logout")]
    Logout = 2,
}

impl Default for ConnectionStaticInfo_State {
    fn default() -> Self {
        Self::Login
    }
}

