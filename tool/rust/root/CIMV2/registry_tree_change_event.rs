// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RegistryTreeChangeEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegistryTreeChangeEvent {
    #[serde(flatten)]
    pub base: RegistryEvent,

/// 
    #[serde(rename = "Hive")]
    pub hive: Option<String>,

/// 
    #[serde(rename = "RootPath")]
    pub root_path: Option<String>,
}

impl RegistryTreeChangeEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RegistryEvent::new(),
            hive: None,
            root_path: None,
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

    /// Sets the value of RootPath
    pub fn set_root_path(&mut self, value: String) {
        self.root_path = Some(value);
    }

    /// Gets the value of RootPath
    pub fn get_root_path(&self) -> Option<&String> {
        self.root_path.as_ref()
    }
}

