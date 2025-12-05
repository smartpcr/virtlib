// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemSettingData_NetworkBootPreferredProtocol
//////////////////////////////////////////////

/// VirtualSystemSettingData_NetworkBootPreferredProtocol enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemSettingData_NetworkBootPreferredProtocol {
    /// IPv4
    #[serde(rename = "IPv4")]
    IPv4 = 4096,
    /// IPv6
    #[serde(rename = "IPv6")]
    IPv6 = 4097,
}

impl Default for VirtualSystemSettingData_NetworkBootPreferredProtocol {
    fn default() -> Self {
        Self::IPv4
    }
}

