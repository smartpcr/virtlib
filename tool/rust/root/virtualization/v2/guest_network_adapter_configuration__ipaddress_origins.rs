// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source GuestNetworkAdapterConfiguration_IPAddressOrigins
//////////////////////////////////////////////

/// GuestNetworkAdapterConfiguration_IPAddressOrigins enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum GuestNetworkAdapterConfiguration_IPAddressOrigins {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Static
    #[serde(rename = "Static")]
    StaticValue = 2,
}

impl Default for GuestNetworkAdapterConfiguration_IPAddressOrigins {
    fn default() -> Self {
        Self::Unknown
    }
}

