// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_System struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_System {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// 
    #[serde(rename = "NameFormat")]
    pub name_format: Option<String>,

/// 
    #[serde(rename = "PrimaryOwnerContact")]
    pub primary_owner_contact: Option<String>,

/// 
    #[serde(rename = "PrimaryOwnerName")]
    pub primary_owner_name: Option<String>,

/// 
    #[serde(rename = "Roles")]
    pub roles: Vec<String>,
}

impl CIM_System {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            creation_class_name: None,
            name_format: None,
            primary_owner_contact: None,
            primary_owner_name: None,
            roles: Vec::new(),
        }
    }


    /// Sets the value of CreationClassName
    pub fn set_creation_class_name(&mut self, value: String) {
        self.creation_class_name = Some(value);
    }

    /// Gets the value of CreationClassName
    pub fn get_creation_class_name(&self) -> Option<&String> {
        self.creation_class_name.as_ref()
    }

    /// Sets the value of NameFormat
    pub fn set_name_format(&mut self, value: String) {
        self.name_format = Some(value);
    }

    /// Gets the value of NameFormat
    pub fn get_name_format(&self) -> Option<&String> {
        self.name_format.as_ref()
    }

    /// Sets the value of PrimaryOwnerContact
    pub fn set_primary_owner_contact(&mut self, value: String) {
        self.primary_owner_contact = Some(value);
    }

    /// Gets the value of PrimaryOwnerContact
    pub fn get_primary_owner_contact(&self) -> Option<&String> {
        self.primary_owner_contact.as_ref()
    }

    /// Sets the value of PrimaryOwnerName
    pub fn set_primary_owner_name(&mut self, value: String) {
        self.primary_owner_name = Some(value);
    }

    /// Gets the value of PrimaryOwnerName
    pub fn get_primary_owner_name(&self) -> Option<&String> {
        self.primary_owner_name.as_ref()
    }

    /// Sets the value of Roles
    pub fn set_roles(&mut self, value: Vec<String>) {
        self.roles = value;
    }

    /// Gets the value of Roles
    pub fn get_roles(&self) -> &Vec<String> {
        &self.roles
    }
}

