// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Registry_TypeGroup1 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Registry_TypeGroup1 {
    #[serde(flatten)]
    pub base: Registry,

/// 
    #[serde(rename = "Index")]
    pub index: Option<u32>,

/// 
    #[serde(rename = "InitialTime")]
    pub initial_time: Option<i64>,

/// 
    #[serde(rename = "KeyHandle")]
    pub key_handle: Option<u32>,

/// 
    #[serde(rename = "KeyName")]
    pub key_name: Option<String>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u32>,
}

impl Registry_TypeGroup1 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Registry::new(),
            index: None,
            initial_time: None,
            key_handle: None,
            key_name: None,
            status: None,
        }
    }


    /// Sets the value of Index
    pub fn set_index(&mut self, value: u32) {
        self.index = Some(value);
    }

    /// Gets the value of Index
    pub fn get_index(&self) -> Option<&u32> {
        self.index.as_ref()
    }

    /// Sets the value of InitialTime
    pub fn set_initial_time(&mut self, value: i64) {
        self.initial_time = Some(value);
    }

    /// Gets the value of InitialTime
    pub fn get_initial_time(&self) -> Option<&i64> {
        self.initial_time.as_ref()
    }

    /// Sets the value of KeyHandle
    pub fn set_key_handle(&mut self, value: u32) {
        self.key_handle = Some(value);
    }

    /// Gets the value of KeyHandle
    pub fn get_key_handle(&self) -> Option<&u32> {
        self.key_handle.as_ref()
    }

    /// Sets the value of KeyName
    pub fn set_key_name(&mut self, value: String) {
        self.key_name = Some(value);
    }

    /// Gets the value of KeyName
    pub fn get_key_name(&self) -> Option<&String> {
        self.key_name.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u32> {
        self.status.as_ref()
    }
}

