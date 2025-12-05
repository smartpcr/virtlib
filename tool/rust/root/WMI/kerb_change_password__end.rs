// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// KerbChangePassword_End struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KerbChangePassword_End {
    #[serde(flatten)]
    pub base: KerbChangePassword,

/// Account Name
    #[serde(rename = "AccountName")]
    pub account_name: Option<String>,

/// Account Realm
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,

/// Status
    #[serde(rename = "Status")]
    pub status: Option<u32>,
}

impl KerbChangePassword_End {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: KerbChangePassword::new(),
            account_name: None,
            domain_name: None,
            status: None,
        }
    }


    /// Sets the value of AccountName
    pub fn set_account_name(&mut self, value: String) {
        self.account_name = Some(value);
    }

    /// Gets the value of AccountName
    pub fn get_account_name(&self) -> Option<&String> {
        self.account_name.as_ref()
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
}

