// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_CompatibleProduct struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_CompatibleProduct {

/// 
    #[serde(rename = "CompatibilityDescription")]
    pub compatibility_description: Option<String>,

/// 
    #[serde(rename = "CompatibleProduct")]
    pub compatible_product: Option<CIM_Product>,

/// 
    #[serde(rename = "Product")]
    pub product: Option<CIM_Product>,
}

impl CIM_CompatibleProduct {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            compatibility_description: None,
            compatible_product: None,
            product: None,
        }
    }


    /// Sets the value of CompatibilityDescription
    pub fn set_compatibility_description(&mut self, value: String) {
        self.compatibility_description = Some(value);
    }

    /// Gets the value of CompatibilityDescription
    pub fn get_compatibility_description(&self) -> Option<&String> {
        self.compatibility_description.as_ref()
    }

    /// Sets the value of CompatibleProduct
    pub fn set_compatible_product(&mut self, value: CIM_Product) {
        self.compatible_product = Some(value);
    }

    /// Gets the value of CompatibleProduct
    pub fn get_compatible_product(&self) -> Option<&CIM_Product> {
        self.compatible_product.as_ref()
    }

    /// Sets the value of Product
    pub fn set_product(&mut self, value: CIM_Product) {
        self.product = Some(value);
    }

    /// Gets the value of Product
    pub fn get_product(&self) -> Option<&CIM_Product> {
        self.product.as_ref()
    }
}

