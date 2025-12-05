// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PageFile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PageFile {
    #[serde(flatten)]
    pub base: CIM_DataFile,

/// 
    #[serde(rename = "FreeSpace")]
    pub free_space: Option<u32>,

/// 
    #[serde(rename = "InitialSize")]
    pub initial_size: Option<u32>,

/// 
    #[serde(rename = "MaximumSize")]
    pub maximum_size: Option<u32>,
}

impl Win32_PageFile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_DataFile::new(),
            free_space: None,
            initial_size: None,
            maximum_size: None,
        }
    }


    /// Sets the value of FreeSpace
    pub fn set_free_space(&mut self, value: u32) {
        self.free_space = Some(value);
    }

    /// Gets the value of FreeSpace
    pub fn get_free_space(&self) -> Option<&u32> {
        self.free_space.as_ref()
    }

    /// Sets the value of InitialSize
    pub fn set_initial_size(&mut self, value: u32) {
        self.initial_size = Some(value);
    }

    /// Gets the value of InitialSize
    pub fn get_initial_size(&self) -> Option<&u32> {
        self.initial_size.as_ref()
    }

    /// Sets the value of MaximumSize
    pub fn set_maximum_size(&mut self, value: u32) {
        self.maximum_size = Some(value);
    }

    /// Gets the value of MaximumSize
    pub fn get_maximum_size(&self) -> Option<&u32> {
        self.maximum_size.as_ref()
    }
}

