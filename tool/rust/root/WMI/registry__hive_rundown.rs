// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Registry_HiveRundown struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Registry_HiveRundown {
    #[serde(flatten)]
    pub base: Registry,

/// 
    #[serde(rename = "FileName")]
    pub file_name: Option<String>,

/// 
    #[serde(rename = "Hive")]
    pub hive: Option<u32>,

/// 
    #[serde(rename = "LinkPath")]
    pub link_path: Option<String>,

/// 
    #[serde(rename = "LoadedKeyCount")]
    pub loaded_key_count: Option<u32>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u64>,
}

impl Registry_HiveRundown {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Registry::new(),
            file_name: None,
            hive: None,
            link_path: None,
            loaded_key_count: None,
            size: None,
        }
    }


    /// Sets the value of FileName
    pub fn set_file_name(&mut self, value: String) {
        self.file_name = Some(value);
    }

    /// Gets the value of FileName
    pub fn get_file_name(&self) -> Option<&String> {
        self.file_name.as_ref()
    }

    /// Sets the value of Hive
    pub fn set_hive(&mut self, value: u32) {
        self.hive = Some(value);
    }

    /// Gets the value of Hive
    pub fn get_hive(&self) -> Option<&u32> {
        self.hive.as_ref()
    }

    /// Sets the value of LinkPath
    pub fn set_link_path(&mut self, value: String) {
        self.link_path = Some(value);
    }

    /// Gets the value of LinkPath
    pub fn get_link_path(&self) -> Option<&String> {
        self.link_path.as_ref()
    }

    /// Sets the value of LoadedKeyCount
    pub fn set_loaded_key_count(&mut self, value: u32) {
        self.loaded_key_count = Some(value);
    }

    /// Gets the value of LoadedKeyCount
    pub fn get_loaded_key_count(&self) -> Option<&u32> {
        self.loaded_key_count.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u64) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u64> {
        self.size.as_ref()
    }
}

