// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Registry_HiveLink struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Registry_HiveLink {
    #[serde(flatten)]
    pub base: Registry,

/// 
    #[serde(rename = "Hive")]
    pub hive: Option<u32>,

/// 
    #[serde(rename = "Path")]
    pub path: Option<String>,
}

impl Registry_HiveLink {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Registry::new(),
            hive: None,
            path: None,
        }
    }


    /// Sets the value of Hive
    pub fn set_hive(&mut self, value: u32) {
        self.hive = Some(value);
    }

    /// Gets the value of Hive
    pub fn get_hive(&self) -> Option<&u32> {
        self.hive.as_ref()
    }

    /// Sets the value of Path
    pub fn set_path(&mut self, value: String) {
        self.path = Some(value);
    }

    /// Gets the value of Path
    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }
}

