// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Registry_Config struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Registry_Config {
    #[serde(flatten)]
    pub base: Registry,

/// 
    #[serde(rename = "CurrentControlSet")]
    pub current_control_set: Option<u32>,
}

impl Registry_Config {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Registry::new(),
            current_control_set: None,
        }
    }


    /// Sets the value of CurrentControlSet
    pub fn set_current_control_set(&mut self, value: u32) {
        self.current_control_set = Some(value);
    }

    /// Gets the value of CurrentControlSet
    pub fn get_current_control_set(&self) -> Option<&u32> {
        self.current_control_set.as_ref()
    }
}

