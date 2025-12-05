// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemSettingData_Vtl2AddressSpaceConfigurationMode
//////////////////////////////////////////////

/// VirtualSystemSettingData_Vtl2AddressSpaceConfigurationMode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemSettingData_Vtl2AddressSpaceConfigurationMode {
    /// Default
    #[serde(rename = "Default")]
    Default = 0,
    /// ConfigurableSizePlacement
    #[serde(rename = "ConfigurableSizePlacement")]
    ConfigurableSizePlacement = 1,
    /// ExplicitPlacement
    #[serde(rename = "ExplicitPlacement")]
    ExplicitPlacement = 2,
}

impl Default for VirtualSystemSettingData_Vtl2AddressSpaceConfigurationMode {
    fn default() -> Self {
        Self::Default
    }
}

