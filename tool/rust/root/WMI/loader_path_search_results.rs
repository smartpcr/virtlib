// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// LoaderPathSearchResults struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoaderPathSearchResults {
    #[serde(flatten)]
    pub base: Image,

/// 
    #[serde(rename = "AppDir")]
    pub app_dir: Option<String>,

/// 
    #[serde(rename = "Cwd")]
    pub cwd: Option<String>,

/// 
    #[serde(rename = "DllDir")]
    pub dll_dir: Option<String>,

/// 
    #[serde(rename = "DllLoadDir")]
    pub dll_load_dir: Option<String>,

/// 
    #[serde(rename = "SearchInfo")]
    pub search_info: Option<u32>,
}

impl LoaderPathSearchResults {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Image::new(),
            app_dir: None,
            cwd: None,
            dll_dir: None,
            dll_load_dir: None,
            search_info: None,
        }
    }


    /// Sets the value of AppDir
    pub fn set_app_dir(&mut self, value: String) {
        self.app_dir = Some(value);
    }

    /// Gets the value of AppDir
    pub fn get_app_dir(&self) -> Option<&String> {
        self.app_dir.as_ref()
    }

    /// Sets the value of Cwd
    pub fn set_cwd(&mut self, value: String) {
        self.cwd = Some(value);
    }

    /// Gets the value of Cwd
    pub fn get_cwd(&self) -> Option<&String> {
        self.cwd.as_ref()
    }

    /// Sets the value of DllDir
    pub fn set_dll_dir(&mut self, value: String) {
        self.dll_dir = Some(value);
    }

    /// Gets the value of DllDir
    pub fn get_dll_dir(&self) -> Option<&String> {
        self.dll_dir.as_ref()
    }

    /// Sets the value of DllLoadDir
    pub fn set_dll_load_dir(&mut self, value: String) {
        self.dll_load_dir = Some(value);
    }

    /// Gets the value of DllLoadDir
    pub fn get_dll_load_dir(&self) -> Option<&String> {
        self.dll_load_dir.as_ref()
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

