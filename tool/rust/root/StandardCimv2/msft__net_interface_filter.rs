// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetInterfaceFilter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetInterfaceFilter {
    #[serde(flatten)]
    pub base: CIM_FilterEntryBase,

/// 
    #[serde(rename = "InterfaceAlias")]
    pub interface_alias: Vec<String>,
}

impl MSFT_NetInterfaceFilter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_FilterEntryBase::new(),
            interface_alias: Vec::new(),
        }
    }


    /// Sets the value of InterfaceAlias
    pub fn set_interface_alias(&mut self, value: Vec<String>) {
        self.interface_alias = value;
    }

    /// Gets the value of InterfaceAlias
    pub fn get_interface_alias(&self) -> &Vec<String> {
        &self.interface_alias
    }
}

