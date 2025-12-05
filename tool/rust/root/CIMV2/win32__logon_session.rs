// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_LogonSession struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_LogonSession {
    #[serde(flatten)]
    pub base: Win32_Session,

/// 
    #[serde(rename = "AuthenticationPackage")]
    pub authentication_package: Option<String>,

/// 
    #[serde(rename = "LogonId")]
    pub logon_id: Option<String>,

/// 
    #[serde(rename = "LogonType")]
    pub logon_type: Option<u32>,
}

impl Win32_LogonSession {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_Session::new(),
            authentication_package: None,
            logon_id: None,
            logon_type: None,
        }
    }


    /// Sets the value of AuthenticationPackage
    pub fn set_authentication_package(&mut self, value: String) {
        self.authentication_package = Some(value);
    }

    /// Gets the value of AuthenticationPackage
    pub fn get_authentication_package(&self) -> Option<&String> {
        self.authentication_package.as_ref()
    }

    /// Sets the value of LogonId
    pub fn set_logon_id(&mut self, value: String) {
        self.logon_id = Some(value);
    }

    /// Gets the value of LogonId
    pub fn get_logon_id(&self) -> Option<&String> {
        self.logon_id.as_ref()
    }

    /// Sets the value of LogonType
    pub fn set_logon_type(&mut self, value: u32) {
        self.logon_type = Some(value);
    }

    /// Gets the value of LogonType
    pub fn get_logon_type(&self) -> Option<&u32> {
        self.logon_type.as_ref()
    }
}

