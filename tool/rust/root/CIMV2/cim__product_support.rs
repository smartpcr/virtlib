// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ProductSupport struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ProductSupport {

/// 
    #[serde(rename = "Product")]
    pub product: Option<CIM_Product>,

/// 
    #[serde(rename = "Support")]
    pub support: Option<CIM_SupportAccess>,
}

impl CIM_ProductSupport {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            product: None,
            support: None,
        }
    }


    /// Sets the value of Product
    pub fn set_product(&mut self, value: CIM_Product) {
        self.product = Some(value);
    }

    /// Gets the value of Product
    pub fn get_product(&self) -> Option<&CIM_Product> {
        self.product.as_ref()
    }

    /// Sets the value of Support
    pub fn set_support(&mut self, value: CIM_SupportAccess) {
        self.support = Some(value);
    }

    /// Gets the value of Support
    pub fn get_support(&self) -> Option<&CIM_SupportAccess> {
        self.support.as_ref()
    }
}

