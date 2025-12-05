// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Cli
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CliConnection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CliConnection {

/// 
    #[serde(rename = "Authority")]
    pub authority: Option<String>,

/// 
    #[serde(rename = "Locale")]
    pub locale: Option<String>,

/// 
    #[serde(rename = "NameSpace")]
    pub name_space: Option<String>,

/// 
    #[serde(rename = "Password")]
    pub password: Option<String>,

/// 
    #[serde(rename = "Server")]
    pub server: Option<String>,

/// 
    #[serde(rename = "User")]
    pub user: Option<String>,
}

impl MSFT_CliConnection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            authority: None,
            locale: None,
            name_space: None,
            password: None,
            server: None,
            user: None,
        }
    }


    /// Sets the value of Authority
    pub fn set_authority(&mut self, value: String) {
        self.authority = Some(value);
    }

    /// Gets the value of Authority
    pub fn get_authority(&self) -> Option<&String> {
        self.authority.as_ref()
    }

    /// Sets the value of Locale
    pub fn set_locale(&mut self, value: String) {
        self.locale = Some(value);
    }

    /// Gets the value of Locale
    pub fn get_locale(&self) -> Option<&String> {
        self.locale.as_ref()
    }

    /// Sets the value of NameSpace
    pub fn set_name_space(&mut self, value: String) {
        self.name_space = Some(value);
    }

    /// Gets the value of NameSpace
    pub fn get_name_space(&self) -> Option<&String> {
        self.name_space.as_ref()
    }

    /// Sets the value of Password
    pub fn set_password(&mut self, value: String) {
        self.password = Some(value);
    }

    /// Gets the value of Password
    pub fn get_password(&self) -> Option<&String> {
        self.password.as_ref()
    }

    /// Sets the value of Server
    pub fn set_server(&mut self, value: String) {
        self.server = Some(value);
    }

    /// Gets the value of Server
    pub fn get_server(&self) -> Option<&String> {
        self.server.as_ref()
    }

    /// Sets the value of User
    pub fn set_user(&mut self, value: String) {
        self.user = Some(value);
    }

    /// Gets the value of User
    pub fn get_user(&self) -> Option<&String> {
        self.user.as_ref()
    }
}

