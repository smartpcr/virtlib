// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RegistryValueChangeEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegistryValueChangeEvent {
    #[serde(flatten)]
    pub base: RegistryEvent,

/// 
    #[serde(rename = "Hive")]
    pub hive: Option<String>,

/// 
    #[serde(rename = "KeyPath")]
    pub key_path: Option<String>,

/// 
    #[serde(rename = "ValueName")]
    pub value_name: Option<String>,
}

impl RegistryValueChangeEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RegistryEvent::new(),
            hive: None,
            key_path: None,
            value_name: None,
        }
    }


    /// Sets the value of Hive
    pub fn set_hive(&mut self, value: String) {
        self.hive = Some(value);
    }

    /// Gets the value of Hive
    pub fn get_hive(&self) -> Option<&String> {
        self.hive.as_ref()
    }

    /// Sets the value of KeyPath
    pub fn set_key_path(&mut self, value: String) {
        self.key_path = Some(value);
    }

    /// Gets the value of KeyPath
    pub fn get_key_path(&self) -> Option<&String> {
        self.key_path.as_ref()
    }

    /// Sets the value of ValueName
    pub fn set_value_name(&mut self, value: String) {
        self.value_name = Some(value);
    }

    /// Gets the value of ValueName
    pub fn get_value_name(&self) -> Option<&String> {
        self.value_name.as_ref()
    }
}

