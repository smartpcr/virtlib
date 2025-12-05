// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source TSDeploymentSettings_GatewayAuthMode
//////////////////////////////////////////////

/// TSDeploymentSettings_GatewayAuthMode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum TSDeploymentSettings_GatewayAuthMode {
    /// Password_0_
    #[serde(rename = "Password_0_")]
    Password0 = 0,
    /// Smartcard_1_
    #[serde(rename = "Smartcard_1_")]
    Smartcard1 = 1,
    /// Allow_User_to_Choose_4_
    #[serde(rename = "Allow_User_to_Choose_4_")]
    AllowUserToChoose4 = 2,
}

impl Default for TSDeploymentSettings_GatewayAuthMode {
    fn default() -> Self {
        Self::Password0
    }
}

