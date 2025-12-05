// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_RegistryPolicySettingBlocked struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_RegistryPolicySettingBlocked {
    #[serde(flatten)]
    pub base: RSOP_PolicySettingBlocked,

/// 
    #[serde(rename = "command")]
    pub command: Option<String>,

/// 
    #[serde(rename = "deleted")]
    pub deleted: Option<bool>,

/// 
    #[serde(rename = "registryKey")]
    pub registry_key: Option<String>,

/// 
    #[serde(rename = "value")]
    pub value: Vec<u8>,

/// 
    #[serde(rename = "valueName")]
    pub value_name: Option<String>,

/// 
    #[serde(rename = "valueType")]
    pub value_type: Option<u32>,
}

impl RSOP_RegistryPolicySettingBlocked {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_PolicySettingBlocked::new(),
            command: None,
            deleted: None,
            registry_key: None,
            value: Vec::new(),
            value_name: None,
            value_type: None,
        }
    }


    /// Sets the value of command
    pub fn set_command(&mut self, value: String) {
        self.command = Some(value);
    }

    /// Gets the value of command
    pub fn get_command(&self) -> Option<&String> {
        self.command.as_ref()
    }

    /// Sets the value of deleted
    pub fn set_deleted(&mut self, value: bool) {
        self.deleted = Some(value);
    }

    /// Gets the value of deleted
    pub fn get_deleted(&self) -> Option<&bool> {
        self.deleted.as_ref()
    }

    /// Sets the value of registryKey
    pub fn set_registry_key(&mut self, value: String) {
        self.registry_key = Some(value);
    }

    /// Gets the value of registryKey
    pub fn get_registry_key(&self) -> Option<&String> {
        self.registry_key.as_ref()
    }

    /// Sets the value of value
    pub fn set_value(&mut self, value: Vec<u8>) {
        self.value = value;
    }

    /// Gets the value of value
    pub fn get_value(&self) -> &Vec<u8> {
        &self.value
    }

    /// Sets the value of valueName
    pub fn set_value_name(&mut self, value: String) {
        self.value_name = Some(value);
    }

    /// Gets the value of valueName
    pub fn get_value_name(&self) -> Option<&String> {
        self.value_name.as_ref()
    }

    /// Sets the value of valueType
    pub fn set_value_type(&mut self, value: u32) {
        self.value_type = Some(value);
    }

    /// Gets the value of valueType
    pub fn get_value_type(&self) -> Option<&u32> {
        self.value_type.as_ref()
    }
}

