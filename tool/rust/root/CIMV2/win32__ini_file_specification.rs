// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_IniFileSpecification struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_IniFileSpecification {
    #[serde(flatten)]
    pub base: CIM_FileSpecification,

/// 
    #[serde(rename = "Action")]
    pub action: Option<u16>,

/// 
    #[serde(rename = "IniFile")]
    pub ini_file: Option<String>,

/// 
    #[serde(rename = "key")]
    pub key: Option<String>,

/// 
    #[serde(rename = "Section")]
    pub section: Option<String>,

/// 
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

impl Win32_IniFileSpecification {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_FileSpecification::new(),
            action: None,
            ini_file: None,
            key: None,
            section: None,
            value: None,
        }
    }


    /// Sets the value of Action
    pub fn set_action(&mut self, value: u16) {
        self.action = Some(value);
    }

    /// Gets the value of Action
    pub fn get_action(&self) -> Option<&u16> {
        self.action.as_ref()
    }

    /// Sets the value of IniFile
    pub fn set_ini_file(&mut self, value: String) {
        self.ini_file = Some(value);
    }

    /// Gets the value of IniFile
    pub fn get_ini_file(&self) -> Option<&String> {
        self.ini_file.as_ref()
    }

    /// Sets the value of key
    pub fn set_key(&mut self, value: String) {
        self.key = Some(value);
    }

    /// Gets the value of key
    pub fn get_key(&self) -> Option<&String> {
        self.key.as_ref()
    }

    /// Sets the value of Section
    pub fn set_section(&mut self, value: String) {
        self.section = Some(value);
    }

    /// Gets the value of Section
    pub fn get_section(&self) -> Option<&String> {
        self.section.as_ref()
    }

    /// Sets the value of Value
    pub fn set_value(&mut self, value: String) {
        self.value = Some(value);
    }

    /// Gets the value of Value
    pub fn get_value(&self) -> Option<&String> {
        self.value.as_ref()
    }
}

