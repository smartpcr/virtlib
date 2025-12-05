// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ProductPhysicalElements struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ProductPhysicalElements {

/// 
    #[serde(rename = "Component")]
    pub component: Option<CIM_PhysicalElement>,

/// 
    #[serde(rename = "Product")]
    pub product: Option<CIM_Product>,
}

impl CIM_ProductPhysicalElements {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            component: None,
            product: None,
        }
    }


    /// Sets the value of Component
    pub fn set_component(&mut self, value: CIM_PhysicalElement) {
        self.component = Some(value);
    }

    /// Gets the value of Component
    pub fn get_component(&self) -> Option<&CIM_PhysicalElement> {
        self.component.as_ref()
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

