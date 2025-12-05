// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source FailoverNetworkAdapterSettingData_ProtocolIFType
//////////////////////////////////////////////

/// FailoverNetworkAdapterSettingData_ProtocolIFType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum FailoverNetworkAdapterSettingData_ProtocolIFType {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// IPv4
    #[serde(rename = "IPv4")]
    IPv4 = 4096,
    /// IPv6
    #[serde(rename = "IPv6")]
    IPv6 = 4097,
    /// IPv4_v6
    #[serde(rename = "IPv4_v6")]
    IPv4V6 = 4098,
}

impl Default for FailoverNetworkAdapterSettingData_ProtocolIFType {
    fn default() -> Self {
        Self::Unknown
    }
}

