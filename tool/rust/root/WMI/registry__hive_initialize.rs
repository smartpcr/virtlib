// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Registry_HiveInitialize struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Registry_HiveInitialize {
    #[serde(flatten)]
    pub base: Registry,

/// 
    #[serde(rename = "FileName")]
    pub file_name: Option<String>,

/// 
    #[serde(rename = "Hive")]
    pub hive: Option<u32>,

/// 
    #[serde(rename = "OperationType")]
    pub operation_type: Option<u32>,

/// 
    #[serde(rename = "PoolTag")]
    pub pool_tag: Option<u32>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u32>,
}

impl Registry_HiveInitialize {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Registry::new(),
            file_name: None,
            hive: None,
            operation_type: None,
            pool_tag: None,
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

    /// Sets the value of OperationType
    pub fn set_operation_type(&mut self, value: u32) {
        self.operation_type = Some(value);
    }

    /// Gets the value of OperationType
    pub fn get_operation_type(&self) -> Option<&u32> {
        self.operation_type.as_ref()
    }

    /// Sets the value of PoolTag
    pub fn set_pool_tag(&mut self, value: u32) {
        self.pool_tag = Some(value);
    }

    /// Gets the value of PoolTag
    pub fn get_pool_tag(&self) -> Option<&u32> {
        self.pool_tag.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u32) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u32> {
        self.size.as_ref()
    }
}

