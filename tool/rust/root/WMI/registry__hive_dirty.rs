// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Registry_HiveDirty struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Registry_HiveDirty {
    #[serde(flatten)]
    pub base: Registry,

/// 
    #[serde(rename = "DirtyReason")]
    pub dirty_reason: Option<u32>,

/// 
    #[serde(rename = "Hive")]
    pub hive: Option<u32>,

/// 
    #[serde(rename = "LinkPath")]
    pub link_path: Option<String>,
}

impl Registry_HiveDirty {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Registry::new(),
            dirty_reason: None,
            hive: None,
            link_path: None,
        }
    }


    /// Sets the value of DirtyReason
    pub fn set_dirty_reason(&mut self, value: u32) {
        self.dirty_reason = Some(value);
    }

    /// Gets the value of DirtyReason
    pub fn get_dirty_reason(&self) -> Option<&u32> {
        self.dirty_reason.as_ref()
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
}

