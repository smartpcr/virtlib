// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemSettingData_ClusterWideNodeCapabilitiesValidationMode
//////////////////////////////////////////////

/// VirtualSystemSettingData_ClusterWideNodeCapabilitiesValidationMode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemSettingData_ClusterWideNodeCapabilitiesValidationMode {
    /// Default
    #[serde(rename = "Default")]
    Default = 0,
    /// Override
    #[serde(rename = "Override")]
    OverrideValue = 1,
}

impl Default for VirtualSystemSettingData_ClusterWideNodeCapabilitiesValidationMode {
    fn default() -> Self {
        Self::Default
    }
}

