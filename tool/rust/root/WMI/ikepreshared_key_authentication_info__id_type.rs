// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source IKEPresharedKeyAuthenticationInfo_IdType
//////////////////////////////////////////////

/// IKEPresharedKeyAuthenticationInfo_IdType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum IKEPresharedKeyAuthenticationInfo_IdType {
    /// ID_IPV4_ADDR
    #[serde(rename = "ID_IPV4_ADDR")]
    IDIPV4ADDR = 1,
    /// ID_FQDN
    #[serde(rename = "ID_FQDN")]
    IDFQDN = 2,
    /// ID_USER_FQDN
    #[serde(rename = "ID_USER_FQDN")]
    IDUSERFQDN = 3,
    /// ID_IPV6_ADDR
    #[serde(rename = "ID_IPV6_ADDR")]
    IDIPV6ADDR = 5,
}

impl Default for IKEPresharedKeyAuthenticationInfo_IdType {
    fn default() -> Self {
        Self::IDIPV4ADDR
    }
}

