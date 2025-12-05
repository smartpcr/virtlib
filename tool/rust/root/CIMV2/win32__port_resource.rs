// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PortResource struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PortResource {
    #[serde(flatten)]
    pub base: Win32_SystemMemoryResource,

/// 
    #[serde(rename = "Alias")]
    pub alias: Option<bool>,
}

impl Win32_PortResource {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_SystemMemoryResource::new(),
            alias: None,
        }
    }


    /// Sets the value of Alias
    pub fn set_alias(&mut self, value: bool) {
        self.alias = Some(value);
    }

    /// Gets the value of Alias
    pub fn get_alias(&self) -> Option<&bool> {
        self.alias.as_ref()
    }
}

