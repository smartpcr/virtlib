// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_RegistryAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_RegistryAction {
    #[serde(flatten)]
    pub base: CIM_Action,

/// 
    #[serde(rename = "EntryName")]
    pub entry_name: Option<String>,

/// 
    #[serde(rename = "EntryValue")]
    pub entry_value: Option<String>,

/// 
    #[serde(rename = "key")]
    pub key: Option<String>,

/// 
    #[serde(rename = "Registry")]
    pub registry: Option<String>,

/// 
    #[serde(rename = "Root")]
    pub root: Option<i16>,
}

impl Win32_RegistryAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Action::new(),
            entry_name: None,
            entry_value: None,
            key: None,
            registry: None,
            root: None,
        }
    }


    /// Sets the value of EntryName
    pub fn set_entry_name(&mut self, value: String) {
        self.entry_name = Some(value);
    }

    /// Gets the value of EntryName
    pub fn get_entry_name(&self) -> Option<&String> {
        self.entry_name.as_ref()
    }

    /// Sets the value of EntryValue
    pub fn set_entry_value(&mut self, value: String) {
        self.entry_value = Some(value);
    }

    /// Gets the value of EntryValue
    pub fn get_entry_value(&self) -> Option<&String> {
        self.entry_value.as_ref()
    }

    /// Sets the value of key
    pub fn set_key(&mut self, value: String) {
        self.key = Some(value);
    }

    /// Gets the value of key
    pub fn get_key(&self) -> Option<&String> {
        self.key.as_ref()
    }

    /// Sets the value of Registry
    pub fn set_registry(&mut self, value: String) {
        self.registry = Some(value);
    }

    /// Gets the value of Registry
    pub fn get_registry(&self) -> Option<&String> {
        self.registry.as_ref()
    }

    /// Sets the value of Root
    pub fn set_root(&mut self, value: i16) {
        self.root = Some(value);
    }

    /// Gets the value of Root
    pub fn get_root(&self) -> Option<&i16> {
        self.root.as_ref()
    }
}

