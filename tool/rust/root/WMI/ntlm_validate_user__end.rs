// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// NtlmValidateUser_End struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NtlmValidateUser_End {
    #[serde(flatten)]
    pub base: NtlmValidateUser,

/// Domain Name
    #[serde(rename = "LogonDomain")]
    pub logon_domain: Option<String>,

/// Logon Server
    #[serde(rename = "LogonServer")]
    pub logon_server: Option<String>,

/// Success Bitmask
    #[serde(rename = "Success")]
    pub success: Option<u32>,

/// User Name
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,

/// Logon Workstation
    #[serde(rename = "Workstation")]
    pub workstation: Option<String>,
}

impl NtlmValidateUser_End {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: NtlmValidateUser::new(),
            logon_domain: None,
            logon_server: None,
            success: None,
            user_name: None,
            workstation: None,
        }
    }


    /// Sets the value of LogonDomain
    pub fn set_logon_domain(&mut self, value: String) {
        self.logon_domain = Some(value);
    }

    /// Gets the value of LogonDomain
    pub fn get_logon_domain(&self) -> Option<&String> {
        self.logon_domain.as_ref()
    }

    /// Sets the value of LogonServer
    pub fn set_logon_server(&mut self, value: String) {
        self.logon_server = Some(value);
    }

    /// Gets the value of LogonServer
    pub fn get_logon_server(&self) -> Option<&String> {
        self.logon_server.as_ref()
    }

    /// Sets the value of Success
    pub fn set_success(&mut self, value: u32) {
        self.success = Some(value);
    }

    /// Gets the value of Success
    pub fn get_success(&self) -> Option<&u32> {
        self.success.as_ref()
    }

    /// Sets the value of UserName
    pub fn set_user_name(&mut self, value: String) {
        self.user_name = Some(value);
    }

    /// Gets the value of UserName
    pub fn get_user_name(&self) -> Option<&String> {
        self.user_name.as_ref()
    }

    /// Sets the value of Workstation
    pub fn set_workstation(&mut self, value: String) {
        self.workstation = Some(value);
    }

    /// Gets the value of Workstation
    pub fn get_workstation(&self) -> Option<&String> {
        self.workstation.as_ref()
    }
}

