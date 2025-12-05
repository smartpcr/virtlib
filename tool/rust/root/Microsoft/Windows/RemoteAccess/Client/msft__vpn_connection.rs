// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_VpnConnection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_VpnConnection {

/// 
    #[serde(rename = "AllUserConnection")]
    pub all_user_connection: Option<bool>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Profile")]
    pub profile: Option<String>,
}

impl MSFT_VpnConnection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            all_user_connection: None,
            name: None,
            profile: None,
        }
    }


    /// Sets the value of AllUserConnection
    pub fn set_all_user_connection(&mut self, value: bool) {
        self.all_user_connection = Some(value);
    }

    /// Gets the value of AllUserConnection
    pub fn get_all_user_connection(&self) -> Option<&bool> {
        self.all_user_connection.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Profile
    pub fn set_profile(&mut self, value: String) {
        self.profile = Some(value);
    }

    /// Gets the value of Profile
    pub fn get_profile(&self) -> Option<&String> {
        self.profile.as_ref()
    }

/// 

    /// * `profile` -  (String)

    /// * `return_value` -  (u32)
    pub fn set(&self, profile: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Profile".to_string(), value: profile.into() });
        self.invoke_method("Set", &args)

    }

}

