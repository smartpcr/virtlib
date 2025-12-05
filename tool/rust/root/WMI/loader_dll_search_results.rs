// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// LoaderDllSearchResults struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoaderDllSearchResults {
    #[serde(flatten)]
    pub base: Image,

/// 
    #[serde(rename = "FullDllName")]
    pub full_dll_name: Option<String>,

/// 
    #[serde(rename = "LdrLoadFlags")]
    pub ldr_load_flags: Option<u32>,

/// 
    #[serde(rename = "LdrSearchFlags")]
    pub ldr_search_flags: Option<u32>,

/// 
    #[serde(rename = "LoadReason")]
    pub load_reason: Option<u32>,

/// 
    #[serde(rename = "SearchInfo")]
    pub search_info: Option<u32>,
}

impl LoaderDllSearchResults {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Image::new(),
            full_dll_name: None,
            ldr_load_flags: None,
            ldr_search_flags: None,
            load_reason: None,
            search_info: None,
        }
    }


    /// Sets the value of FullDllName
    pub fn set_full_dll_name(&mut self, value: String) {
        self.full_dll_name = Some(value);
    }

    /// Gets the value of FullDllName
    pub fn get_full_dll_name(&self) -> Option<&String> {
        self.full_dll_name.as_ref()
    }

    /// Sets the value of LdrLoadFlags
    pub fn set_ldr_load_flags(&mut self, value: u32) {
        self.ldr_load_flags = Some(value);
    }

    /// Gets the value of LdrLoadFlags
    pub fn get_ldr_load_flags(&self) -> Option<&u32> {
        self.ldr_load_flags.as_ref()
    }

    /// Sets the value of LdrSearchFlags
    pub fn set_ldr_search_flags(&mut self, value: u32) {
        self.ldr_search_flags = Some(value);
    }

    /// Gets the value of LdrSearchFlags
    pub fn get_ldr_search_flags(&self) -> Option<&u32> {
        self.ldr_search_flags.as_ref()
    }

    /// Sets the value of LoadReason
    pub fn set_load_reason(&mut self, value: u32) {
        self.load_reason = Some(value);
    }

    /// Gets the value of LoadReason
    pub fn get_load_reason(&self) -> Option<&u32> {
        self.load_reason.as_ref()
    }

    /// Sets the value of SearchInfo
    pub fn set_search_info(&mut self, value: u32) {
        self.search_info = Some(value);
    }

    /// Gets the value of SearchInfo
    pub fn get_search_info(&self) -> Option<&u32> {
        self.search_info.as_ref()
    }
}

