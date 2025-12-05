// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetImPlatProvider struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetImPlatProvider {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 17
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl MSFT_NetImPlatProvider {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            name: None,
        }
    }


    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

