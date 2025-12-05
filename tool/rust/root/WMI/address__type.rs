// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Address_Type
//////////////////////////////////////////////

/// Address_Type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Address_Type {
    /// Text_Address
    #[serde(rename = "Text_Address")]
    TextAddress = 0,
    /// IpV4_Address
    #[serde(rename = "IpV4_Address")]
    IpV4Address = 1,
    /// IpV6_Address
    #[serde(rename = "IpV6_Address")]
    IpV6Address = 2,
    /// Empty_Address
    #[serde(rename = "Empty_Address")]
    EmptyAddress = 3,
}

impl Default for Address_Type {
    fn default() -> Self {
        Self::TextAddress
    }
}

