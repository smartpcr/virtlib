// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ProductCheck struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ProductCheck {

/// 
    #[serde(rename = "Check")]
    pub check: Option<CIM_Check>,

/// 
    #[serde(rename = "Product")]
    pub product: Option<Win32_Product>,
}

impl Win32_ProductCheck {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            check: None,
            product: None,
        }
    }


    /// Sets the value of Check
    pub fn set_check(&mut self, value: CIM_Check) {
        self.check = Some(value);
    }

    /// Gets the value of Check
    pub fn get_check(&self) -> Option<&CIM_Check> {
        self.check.as_ref()
    }

    /// Sets the value of Product
    pub fn set_product(&mut self, value: Win32_Product) {
        self.product = Some(value);
    }

    /// Gets the value of Product
    pub fn get_product(&self) -> Option<&Win32_Product> {
        self.product.as_ref()
    }
}

