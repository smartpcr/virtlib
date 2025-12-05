// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetApplicationFilter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetApplicationFilter {
    #[serde(flatten)]
    pub base: CIM_FilterEntryBase,

/// 
    #[serde(rename = "AppPath")]
    pub app_path: Option<String>,

/// 
    #[serde(rename = "Package")]
    pub package: Option<String>,
}

impl MSFT_NetApplicationFilter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_FilterEntryBase::new(),
            app_path: None,
            package: None,
        }
    }


    /// Sets the value of AppPath
    pub fn set_app_path(&mut self, value: String) {
        self.app_path = Some(value);
    }

    /// Gets the value of AppPath
    pub fn get_app_path(&self) -> Option<&String> {
        self.app_path.as_ref()
    }

    /// Sets the value of Package
    pub fn set_package(&mut self, value: String) {
        self.package = Some(value);
    }

    /// Gets the value of Package
    pub fn get_package(&self) -> Option<&String> {
        self.package.as_ref()
    }
}

