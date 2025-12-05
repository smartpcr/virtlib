// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_Credential struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_Credential {

/// 
    #[serde(rename = "Password")]
    pub password: Option<String>,

/// 
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,
}

impl MSFT_Credential {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            password: None,
            user_name: None,
        }
    }


    /// Sets the value of Password
    pub fn set_password(&mut self, value: String) {
        self.password = Some(value);
    }

    /// Gets the value of Password
    pub fn get_password(&self) -> Option<&String> {
        self.password.as_ref()
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

