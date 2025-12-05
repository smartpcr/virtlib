// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetInterfaceTypeFilter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetInterfaceTypeFilter {
    #[serde(flatten)]
    pub base: CIM_FilterEntryBase,

/// 
    #[serde(rename = "InterfaceType")]
    pub interface_type: Option<u32>,
}

impl MSFT_NetInterfaceTypeFilter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_FilterEntryBase::new(),
            interface_type: None,
        }
    }


    /// Sets the value of InterfaceType
    pub fn set_interface_type(&mut self, value: u32) {
        self.interface_type = Some(value);
    }

    /// Gets the value of InterfaceType
    pub fn get_interface_type(&self) -> Option<&u32> {
        self.interface_type.as_ref()
    }
}

