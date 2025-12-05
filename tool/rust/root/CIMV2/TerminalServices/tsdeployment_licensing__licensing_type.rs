// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source TSDeploymentLicensing_LicensingType
//////////////////////////////////////////////

/// TSDeploymentLicensing_LicensingType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum TSDeploymentLicensing_LicensingType {
    /// Per_Device
    #[serde(rename = "Per_Device")]
    PerDevice = 2,
    /// Per_User
    #[serde(rename = "Per_User")]
    PerUser = 4,
    /// Not_Yet_Configured
    #[serde(rename = "Not_Yet_Configured")]
    NotYetConfigured = 5,
}

impl Default for TSDeploymentLicensing_LicensingType {
    fn default() -> Self {
        Self::PerDevice
    }
}

