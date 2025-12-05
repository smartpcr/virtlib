// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source IKEPresharedKeyAuthenticationInfo_AuthMethod
//////////////////////////////////////////////

/// IKEPresharedKeyAuthenticationInfo_AuthMethod enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum IKEPresharedKeyAuthenticationInfo_AuthMethod {
    /// IKE_AUTHENTICATION_PRESHARED_KEY_METHOD
    #[serde(rename = "IKE_AUTHENTICATION_PRESHARED_KEY_METHOD")]
    IKEAUTHENTICATIONPRESHAREDKEYMETHOD = 1,
}

impl Default for IKEPresharedKeyAuthenticationInfo_AuthMethod {
    fn default() -> Self {
        Self::IKEAUTHENTICATIONPRESHAREDKEYMETHOD
    }
}

