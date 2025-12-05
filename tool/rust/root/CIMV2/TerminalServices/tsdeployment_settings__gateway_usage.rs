// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source TSDeploymentSettings_GatewayUsage
//////////////////////////////////////////////

/// TSDeploymentSettings_GatewayUsage enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum TSDeploymentSettings_GatewayUsage {
    /// NoGateway
    #[serde(rename = "NoGateway")]
    NoGateway = 0,
    /// UseGatewayBypassLocal
    #[serde(rename = "UseGatewayBypassLocal")]
    UseGatewayBypassLocal = 1,
    /// UseGateway
    #[serde(rename = "UseGateway")]
    UseGateway = 2,
    /// DetectGateway
    #[serde(rename = "DetectGateway")]
    DetectGateway = 3,
}

impl Default for TSDeploymentSettings_GatewayUsage {
    fn default() -> Self {
        Self::NoGateway
    }
}

