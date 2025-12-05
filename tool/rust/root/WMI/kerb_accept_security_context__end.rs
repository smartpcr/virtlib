// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// KerbAcceptSecurityContext_End struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KerbAcceptSecurityContext_End {
    #[serde(flatten)]
    pub base: KerbAcceptSecurityContext,

/// Credentials Source
    #[serde(rename = "CredSource")]
    pub cred_source: Option<String>,

/// Domain Name
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,

/// Status
    #[serde(rename = "Status")]
    pub status: Option<u32>,

/// Target
    #[serde(rename = "Target")]
    pub target: Option<String>,

/// User Name
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,
}

impl KerbAcceptSecurityContext_End {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: KerbAcceptSecurityContext::new(),
            cred_source: None,
            domain_name: None,
            status: None,
            target: None,
            user_name: None,
        }
    }


    /// Sets the value of CredSource
    pub fn set_cred_source(&mut self, value: String) {
        self.cred_source = Some(value);
    }

    /// Gets the value of CredSource
    pub fn get_cred_source(&self) -> Option<&String> {
        self.cred_source.as_ref()
    }

    /// Sets the value of DomainName
    pub fn set_domain_name(&mut self, value: String) {
        self.domain_name = Some(value);
    }

    /// Gets the value of DomainName
    pub fn get_domain_name(&self) -> Option<&String> {
        self.domain_name.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u32> {
        self.status.as_ref()
    }

    /// Sets the value of Target
    pub fn set_target(&mut self, value: String) {
        self.target = Some(value);
    }

    /// Gets the value of Target
    pub fn get_target(&self) -> Option<&String> {
        self.target.as_ref()
    }

    /// Sets the value of UserName
    pub fn set_user_name(&mut self, value: String) {
        self.user_name = Some(value);
    }

    /// Gets the value of UserName
    pub fn get_user_name(&self) -> Option<&String> {
        self.user_name.as_ref()
    }
}

