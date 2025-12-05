// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_StartupCommand struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_StartupCommand {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "Command")]
    pub command: Option<String>,

/// 
    #[serde(rename = "Location")]
    pub location: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "User")]
    pub user: Option<String>,

/// 
    #[serde(rename = "UserSID")]
    pub user_sid: Option<String>,
}

impl Win32_StartupCommand {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            command: None,
            location: None,
            name: None,
            user: None,
            user_sid: None,
        }
    }


    /// Sets the value of Command
    pub fn set_command(&mut self, value: String) {
        self.command = Some(value);
    }

    /// Gets the value of Command
    pub fn get_command(&self) -> Option<&String> {
        self.command.as_ref()
    }

    /// Sets the value of Location
    pub fn set_location(&mut self, value: String) {
        self.location = Some(value);
    }

    /// Gets the value of Location
    pub fn get_location(&self) -> Option<&String> {
        self.location.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of User
    pub fn set_user(&mut self, value: String) {
        self.user = Some(value);
    }

    /// Gets the value of User
    pub fn get_user(&self) -> Option<&String> {
        self.user.as_ref()
    }

    /// Sets the value of UserSID
    pub fn set_user_sid(&mut self, value: String) {
        self.user_sid = Some(value);
    }

    /// Gets the value of UserSID
    pub fn get_user_sid(&self) -> Option<&String> {
        self.user_sid.as_ref()
    }
}

