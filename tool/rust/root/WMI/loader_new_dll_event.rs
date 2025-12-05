// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// LoaderNewDllEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoaderNewDllEvent {
    #[serde(flatten)]
    pub base: Image,

/// 
    #[serde(rename = "FilePath")]
    pub file_path: Option<String>,

/// 
    #[serde(rename = "LoadReason")]
    pub load_reason: Option<u32>,

/// 
    #[serde(rename = "NewDllBaseAddress")]
    pub new_dll_base_address: Option<u32>,

/// 
    #[serde(rename = "ParentDllBaseAddress")]
    pub parent_dll_base_address: Option<u32>,
}

impl LoaderNewDllEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Image::new(),
            file_path: None,
            load_reason: None,
            new_dll_base_address: None,
            parent_dll_base_address: None,
        }
    }


    /// Sets the value of FilePath
    pub fn set_file_path(&mut self, value: String) {
        self.file_path = Some(value);
    }

    /// Gets the value of FilePath
    pub fn get_file_path(&self) -> Option<&String> {
        self.file_path.as_ref()
    }

    /// Sets the value of LoadReason
    pub fn set_load_reason(&mut self, value: u32) {
        self.load_reason = Some(value);
    }

    /// Gets the value of LoadReason
    pub fn get_load_reason(&self) -> Option<&u32> {
        self.load_reason.as_ref()
    }

    /// Sets the value of NewDllBaseAddress
    pub fn set_new_dll_base_address(&mut self, value: u32) {
        self.new_dll_base_address = Some(value);
    }

    /// Gets the value of NewDllBaseAddress
    pub fn get_new_dll_base_address(&self) -> Option<&u32> {
        self.new_dll_base_address.as_ref()
    }

    /// Sets the value of ParentDllBaseAddress
    pub fn set_parent_dll_base_address(&mut self, value: u32) {
        self.parent_dll_base_address = Some(value);
    }

    /// Gets the value of ParentDllBaseAddress
    pub fn get_parent_dll_base_address(&self) -> Option<&u32> {
        self.parent_dll_base_address.as_ref()
    }
}

