// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ServerConnection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ServerConnection {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "ActiveTime")]
    pub active_time: Option<u32>,

/// 
    #[serde(rename = "ComputerName")]
    pub computer_name: Option<String>,

/// 
    #[serde(rename = "ConnectionID")]
    pub connection_id: Option<u32>,

/// 
    #[serde(rename = "NumberOfFiles")]
    pub number_of_files: Option<u32>,

/// 
    #[serde(rename = "NumberOfUsers")]
    pub number_of_users: Option<u32>,

/// 
    #[serde(rename = "ShareName")]
    pub share_name: Option<String>,

/// 
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,
}

impl Win32_ServerConnection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            active_time: None,
            computer_name: None,
            connection_id: None,
            number_of_files: None,
            number_of_users: None,
            share_name: None,
            user_name: None,
        }
    }


    /// Sets the value of ActiveTime
    pub fn set_active_time(&mut self, value: u32) {
        self.active_time = Some(value);
    }

    /// Gets the value of ActiveTime
    pub fn get_active_time(&self) -> Option<&u32> {
        self.active_time.as_ref()
    }

    /// Sets the value of ComputerName
    pub fn set_computer_name(&mut self, value: String) {
        self.computer_name = Some(value);
    }

    /// Gets the value of ComputerName
    pub fn get_computer_name(&self) -> Option<&String> {
        self.computer_name.as_ref()
    }

    /// Sets the value of ConnectionID
    pub fn set_connection_id(&mut self, value: u32) {
        self.connection_id = Some(value);
    }

    /// Gets the value of ConnectionID
    pub fn get_connection_id(&self) -> Option<&u32> {
        self.connection_id.as_ref()
    }

    /// Sets the value of NumberOfFiles
    pub fn set_number_of_files(&mut self, value: u32) {
        self.number_of_files = Some(value);
    }

    /// Gets the value of NumberOfFiles
    pub fn get_number_of_files(&self) -> Option<&u32> {
        self.number_of_files.as_ref()
    }

    /// Sets the value of NumberOfUsers
    pub fn set_number_of_users(&mut self, value: u32) {
        self.number_of_users = Some(value);
    }

    /// Gets the value of NumberOfUsers
    pub fn get_number_of_users(&self) -> Option<&u32> {
        self.number_of_users.as_ref()
    }

    /// Sets the value of ShareName
    pub fn set_share_name(&mut self, value: String) {
        self.share_name = Some(value);
    }

    /// Gets the value of ShareName
    pub fn get_share_name(&self) -> Option<&String> {
        self.share_name.as_ref()
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

