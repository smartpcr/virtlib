// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_WebApplication struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_WebApplication {

/// 
    #[serde(rename = "PackageName")]
    pub package_name: Option<String>,

/// 
    #[serde(rename = "PackageUrl")]
    pub package_url: Option<String>,

/// 
    #[serde(rename = "ShortcutFilename")]
    pub shortcut_filename: Option<String>,

/// 
    #[serde(rename = "ShortcutFolder")]
    pub shortcut_folder: Option<String>,
}

impl MDM_WebApplication {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            package_name: None,
            package_url: None,
            shortcut_filename: None,
            shortcut_folder: None,
        }
    }


    /// Sets the value of PackageName
    pub fn set_package_name(&mut self, value: String) {
        self.package_name = Some(value);
    }

    /// Gets the value of PackageName
    pub fn get_package_name(&self) -> Option<&String> {
        self.package_name.as_ref()
    }

    /// Sets the value of PackageUrl
    pub fn set_package_url(&mut self, value: String) {
        self.package_url = Some(value);
    }

    /// Gets the value of PackageUrl
    pub fn get_package_url(&self) -> Option<&String> {
        self.package_url.as_ref()
    }

    /// Sets the value of ShortcutFilename
    pub fn set_shortcut_filename(&mut self, value: String) {
        self.shortcut_filename = Some(value);
    }

    /// Gets the value of ShortcutFilename
    pub fn get_shortcut_filename(&self) -> Option<&String> {
        self.shortcut_filename.as_ref()
    }

    /// Sets the value of ShortcutFolder
    pub fn set_shortcut_folder(&mut self, value: String) {
        self.shortcut_folder = Some(value);
    }

    /// Gets the value of ShortcutFolder
    pub fn get_shortcut_folder(&self) -> Option<&String> {
        self.shortcut_folder.as_ref()
    }
}

