// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_BaseBoard struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_BaseBoard {
    #[serde(flatten)]
    pub base: CIM_Card,

/// 
    #[serde(rename = "ConfigOptions")]
    pub config_options: Vec<String>,

/// 
    #[serde(rename = "Product")]
    pub product: Option<String>,
}

impl Win32_BaseBoard {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Card::new(),
            config_options: Vec::new(),
            product: None,
        }
    }


    /// Sets the value of ConfigOptions
    pub fn set_config_options(&mut self, value: Vec<String>) {
        self.config_options = value;
    }

    /// Gets the value of ConfigOptions
    pub fn get_config_options(&self) -> &Vec<String> {
        &self.config_options
    }

    /// Sets the value of Product
    pub fn set_product(&mut self, value: String) {
        self.product = Some(value);
    }

    /// Gets the value of Product
    pub fn get_product(&self) -> Option<&String> {
        self.product.as_ref()
    }
}

