// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// NtlmLogonUser_End struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NtlmLogonUser_End {
    #[serde(flatten)]
    pub base: NtlmLogonUser,

/// Domain Name
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,

/// Logon Type
    #[serde(rename = "LogonType")]
    pub logon_type: Option<u32>,

/// Status
    #[serde(rename = "Status")]
    pub status: Option<u32>,

/// User Name
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,
}

impl NtlmLogonUser_End {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: NtlmLogonUser::new(),
            domain_name: None,
            logon_type: None,
            status: None,
            user_name: None,
        }
    }


    /// Sets the value of DomainName
    pub fn set_domain_name(&mut self, value: String) {
        self.domain_name = Some(value);
    }

    /// Gets the value of DomainName
    pub fn get_domain_name(&self) -> Option<&String> {
        self.domain_name.as_ref()
    }

    /// Sets the value of LogonType
    pub fn set_logon_type(&mut self, value: u32) {
        self.logon_type = Some(value);
    }

    /// Gets the value of LogonType
    pub fn get_logon_type(&self) -> Option<&u32> {
        self.logon_type.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u32> {
        self.status.as_ref()
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

