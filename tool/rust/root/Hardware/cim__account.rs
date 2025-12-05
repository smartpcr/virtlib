// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Hardware
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Account struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Account {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// 
    #[serde(rename = "Descriptions")]
    pub descriptions: Vec<String>,

/// 
    #[serde(rename = "Host")]
    pub host: Vec<String>,

/// 
    #[serde(rename = "LocalityName")]
    pub locality_name: Vec<String>,

/// 
    #[serde(rename = "ObjectClass")]
    pub object_class: Vec<String>,

/// 
    #[serde(rename = "OrganizationName")]
    pub organization_name: Vec<String>,

/// 
    #[serde(rename = "OU")]
    pub ou: Vec<String>,

/// 
    #[serde(rename = "SeeAlso")]
    pub see_also: Vec<String>,

/// 
    #[serde(rename = "SystemCreationClassName")]
    pub system_creation_class_name: Option<String>,

/// 
    #[serde(rename = "SystemName")]
    pub system_name: Option<String>,

/// 
    #[serde(rename = "UserCertificate")]
    pub user_certificate: Vec<String>,

/// 
    #[serde(rename = "UserID")]
    pub user_id: Option<String>,

/// 
    #[serde(rename = "UserPassword")]
    pub user_password: Vec<String>,
}

impl CIM_Account {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            creation_class_name: None,
            descriptions: Vec::new(),
            host: Vec::new(),
            locality_name: Vec::new(),
            object_class: Vec::new(),
            organization_name: Vec::new(),
            ou: Vec::new(),
            see_also: Vec::new(),
            system_creation_class_name: None,
            system_name: None,
            user_certificate: Vec::new(),
            user_id: None,
            user_password: Vec::new(),
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

    /// Sets the value of Descriptions
    pub fn set_descriptions(&mut self, value: Vec<String>) {
        self.descriptions = value;
    }

    /// Gets the value of Descriptions
    pub fn get_descriptions(&self) -> &Vec<String> {
        &self.descriptions
    }

    /// Sets the value of Host
    pub fn set_host(&mut self, value: Vec<String>) {
        self.host = value;
    }

    /// Gets the value of Host
    pub fn get_host(&self) -> &Vec<String> {
        &self.host
    }

    /// Sets the value of LocalityName
    pub fn set_locality_name(&mut self, value: Vec<String>) {
        self.locality_name = value;
    }

    /// Gets the value of LocalityName
    pub fn get_locality_name(&self) -> &Vec<String> {
        &self.locality_name
    }

    /// Sets the value of ObjectClass
    pub fn set_object_class(&mut self, value: Vec<String>) {
        self.object_class = value;
    }

    /// Gets the value of ObjectClass
    pub fn get_object_class(&self) -> &Vec<String> {
        &self.object_class
    }

    /// Sets the value of OrganizationName
    pub fn set_organization_name(&mut self, value: Vec<String>) {
        self.organization_name = value;
    }

    /// Gets the value of OrganizationName
    pub fn get_organization_name(&self) -> &Vec<String> {
        &self.organization_name
    }

    /// Sets the value of OU
    pub fn set_ou(&mut self, value: Vec<String>) {
        self.ou = value;
    }

    /// Gets the value of OU
    pub fn get_ou(&self) -> &Vec<String> {
        &self.ou
    }

    /// Sets the value of SeeAlso
    pub fn set_see_also(&mut self, value: Vec<String>) {
        self.see_also = value;
    }

    /// Gets the value of SeeAlso
    pub fn get_see_also(&self) -> &Vec<String> {
        &self.see_also
    }

    /// Sets the value of SystemCreationClassName
    pub fn set_system_creation_class_name(&mut self, value: String) {
        self.system_creation_class_name = Some(value);
    }

    /// Gets the value of SystemCreationClassName
    pub fn get_system_creation_class_name(&self) -> Option<&String> {
        self.system_creation_class_name.as_ref()
    }

    /// Sets the value of SystemName
    pub fn set_system_name(&mut self, value: String) {
        self.system_name = Some(value);
    }

    /// Gets the value of SystemName
    pub fn get_system_name(&self) -> Option<&String> {
        self.system_name.as_ref()
    }

    /// Sets the value of UserCertificate
    pub fn set_user_certificate(&mut self, value: Vec<String>) {
        self.user_certificate = value;
    }

    /// Gets the value of UserCertificate
    pub fn get_user_certificate(&self) -> &Vec<String> {
        &self.user_certificate
    }

    /// Sets the value of UserID
    pub fn set_user_id(&mut self, value: String) {
        self.user_id = Some(value);
    }

    /// Gets the value of UserID
    pub fn get_user_id(&self) -> Option<&String> {
        self.user_id.as_ref()
    }

    /// Sets the value of UserPassword
    pub fn set_user_password(&mut self, value: Vec<String>) {
        self.user_password = value;
    }

    /// Gets the value of UserPassword
    pub fn get_user_password(&self) -> &Vec<String> {
        &self.user_password
    }
}

