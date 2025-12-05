// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RegistryKeyChangeEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegistryKeyChangeEvent {
    #[serde(flatten)]
    pub base: RegistryEvent,

/// 
    #[serde(rename = "Hive")]
    pub hive: Option<String>,

/// 
    #[serde(rename = "KeyPath")]
    pub key_path: Option<String>,
}

impl RegistryKeyChangeEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RegistryEvent::new(),
            hive: None,
            key_path: None,
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
}

