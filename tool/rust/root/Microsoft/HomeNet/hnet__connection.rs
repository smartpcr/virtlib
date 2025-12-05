// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.HomeNet
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HNet_Connection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HNet_Connection {

/// 
    #[serde(rename = "Guid")]
    pub guid: Option<String>,

/// 
    #[serde(rename = "IsLanConnection")]
    pub is_lan_connection: Option<bool>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "PhonebookPath")]
    pub phonebook_path: Option<String>,
}

impl HNet_Connection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            guid: None,
            is_lan_connection: None,
            name: None,
            phonebook_path: None,
        }
    }


    /// Sets the value of Guid
    pub fn set_guid(&mut self, value: String) {
        self.guid = Some(value);
    }

    /// Gets the value of Guid
    pub fn get_guid(&self) -> Option<&String> {
        self.guid.as_ref()
    }

    /// Sets the value of IsLanConnection
    pub fn set_is_lan_connection(&mut self, value: bool) {
        self.is_lan_connection = Some(value);
    }

    /// Gets the value of IsLanConnection
    pub fn get_is_lan_connection(&self) -> Option<&bool> {
        self.is_lan_connection.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of PhonebookPath
    pub fn set_phonebook_path(&mut self, value: String) {
        self.phonebook_path = Some(value);
    }

    /// Gets the value of PhonebookPath
    pub fn get_phonebook_path(&self) -> Option<&String> {
        self.phonebook_path.as_ref()
    }
}

