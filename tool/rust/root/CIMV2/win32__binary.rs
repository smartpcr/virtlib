// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_Binary struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_Binary {
    #[serde(flatten)]
    pub base: Win32_MSIResource,

/// 
    #[serde(rename = "Data")]
    pub data: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "ProductCode")]
    pub product_code: Option<String>,
}

impl Win32_Binary {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_MSIResource::new(),
            data: None,
            name: None,
            product_code: None,
        }
    }


    /// Sets the value of Data
    pub fn set_data(&mut self, value: String) {
        self.data = Some(value);
    }

    /// Gets the value of Data
    pub fn get_data(&self) -> Option<&String> {
        self.data.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of ProductCode
    pub fn set_product_code(&mut self, value: String) {
        self.product_code = Some(value);
    }

    /// Gets the value of ProductCode
    pub fn get_product_code(&self) -> Option<&String> {
        self.product_code.as_ref()
    }
}

