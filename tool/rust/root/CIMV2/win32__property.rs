// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_Property struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_Property {
    #[serde(flatten)]
    pub base: Win32_MSIResource,

/// 
    #[serde(rename = "ProductCode")]
    pub product_code: Option<String>,

/// 
    #[serde(rename = "Property")]
    pub property: Option<String>,

/// 
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

impl Win32_Property {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_MSIResource::new(),
            product_code: None,
            property: None,
            value: None,
        }
    }


    /// Sets the value of ProductCode
    pub fn set_product_code(&mut self, value: String) {
        self.product_code = Some(value);
    }

    /// Gets the value of ProductCode
    pub fn get_product_code(&self) -> Option<&String> {
        self.product_code.as_ref()
    }

    /// Sets the value of Property
    pub fn set_property(&mut self, value: String) {
        self.property = Some(value);
    }

    /// Gets the value of Property
    pub fn get_property(&self) -> Option<&String> {
        self.property.as_ref()
    }

    /// Sets the value of Value
    pub fn set_value(&mut self, value: String) {
        self.value = Some(value);
    }

    /// Gets the value of Value
    pub fn get_value(&self) -> Option<&String> {
        self.value.as_ref()
    }
}

