// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DSCMethodInvoked struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DSCMethodInvoked {

/// 
    #[serde(rename = "ConfigurationData")]
    pub configuration_data: Vec<u8>,

/// 
    #[serde(rename = "ConfigurationNumber")]
    pub configuration_number: Option<u8>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "Guid")]
    pub guid: Option<String>,

/// 
    #[serde(rename = "MethodNumber")]
    pub method_number: Option<u8>,

/// 
    #[serde(rename = "ModuleName")]
    pub module_name: Option<String>,

/// 
    #[serde(rename = "ResourceName")]
    pub resource_name: Option<String>,

/// 
    #[serde(rename = "UserSid")]
    pub user_sid: Option<String>,
}

impl MSFT_DSCMethodInvoked {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            configuration_data: Vec::new(),
            configuration_number: None,
            flags: None,
            guid: None,
            method_number: None,
            module_name: None,
            resource_name: None,
            user_sid: None,
        }
    }


    /// Sets the value of ConfigurationData
    pub fn set_configuration_data(&mut self, value: Vec<u8>) {
        self.configuration_data = value;
    }

    /// Gets the value of ConfigurationData
    pub fn get_configuration_data(&self) -> &Vec<u8> {
        &self.configuration_data
    }

    /// Sets the value of ConfigurationNumber
    pub fn set_configuration_number(&mut self, value: u8) {
        self.configuration_number = Some(value);
    }

    /// Gets the value of ConfigurationNumber
    pub fn get_configuration_number(&self) -> Option<&u8> {
        self.configuration_number.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of Guid
    pub fn set_guid(&mut self, value: String) {
        self.guid = Some(value);
    }

    /// Gets the value of Guid
    pub fn get_guid(&self) -> Option<&String> {
        self.guid.as_ref()
    }

    /// Sets the value of MethodNumber
    pub fn set_method_number(&mut self, value: u8) {
        self.method_number = Some(value);
    }

    /// Gets the value of MethodNumber
    pub fn get_method_number(&self) -> Option<&u8> {
        self.method_number.as_ref()
    }

    /// Sets the value of ModuleName
    pub fn set_module_name(&mut self, value: String) {
        self.module_name = Some(value);
    }

    /// Gets the value of ModuleName
    pub fn get_module_name(&self) -> Option<&String> {
        self.module_name.as_ref()
    }

    /// Sets the value of ResourceName
    pub fn set_resource_name(&mut self, value: String) {
        self.resource_name = Some(value);
    }

    /// Gets the value of ResourceName
    pub fn get_resource_name(&self) -> Option<&String> {
        self.resource_name.as_ref()
    }

    /// Sets the value of UserSid
    pub fn set_user_sid(&mut self, value: String) {
        self.user_sid = Some(value);
    }

    /// Gets the value of UserSid
    pub fn get_user_sid(&self) -> Option<&String> {
        self.user_sid.as_ref()
    }
}

