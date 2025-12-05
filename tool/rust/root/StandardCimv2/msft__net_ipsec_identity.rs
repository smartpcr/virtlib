// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetIPsecIdentity struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetIPsecIdentity {

/// 
    #[serde(rename = "AuthenticationMethod")]
    pub authentication_method: Option<u32>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "Identity")]
    pub identity: Option<String>,

/// 
    #[serde(rename = "ImpersonationType")]
    pub impersonation_type: Option<u32>,
}

impl MSFT_NetIPsecIdentity {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            authentication_method: None,
            flags: None,
            identity: None,
            impersonation_type: None,
        }
    }


    /// Sets the value of AuthenticationMethod
    pub fn set_authentication_method(&mut self, value: u32) {
        self.authentication_method = Some(value);
    }

    /// Gets the value of AuthenticationMethod
    pub fn get_authentication_method(&self) -> Option<&u32> {
        self.authentication_method.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of Identity
    pub fn set_identity(&mut self, value: String) {
        self.identity = Some(value);
    }

    /// Gets the value of Identity
    pub fn get_identity(&self) -> Option<&String> {
        self.identity.as_ref()
    }

    /// Sets the value of ImpersonationType
    pub fn set_impersonation_type(&mut self, value: u32) {
        self.impersonation_type = Some(value);
    }

    /// Gets the value of ImpersonationType
    pub fn get_impersonation_type(&self) -> Option<&u32> {
        self.impersonation_type.as_ref()
    }
}

