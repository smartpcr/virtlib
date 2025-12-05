// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ProductProductDependency struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ProductProductDependency {

/// 
    #[serde(rename = "DependentProduct")]
    pub dependent_product: Option<CIM_Product>,

/// 
    #[serde(rename = "RequiredProduct")]
    pub required_product: Option<CIM_Product>,

/// 
    #[serde(rename = "TypeOfDependency")]
    pub type_of_dependency: Option<u16>,
}

impl CIM_ProductProductDependency {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            dependent_product: None,
            required_product: None,
            type_of_dependency: None,
        }
    }


    /// Sets the value of DependentProduct
    pub fn set_dependent_product(&mut self, value: CIM_Product) {
        self.dependent_product = Some(value);
    }

    /// Gets the value of DependentProduct
    pub fn get_dependent_product(&self) -> Option<&CIM_Product> {
        self.dependent_product.as_ref()
    }

    /// Sets the value of RequiredProduct
    pub fn set_required_product(&mut self, value: CIM_Product) {
        self.required_product = Some(value);
    }

    /// Gets the value of RequiredProduct
    pub fn get_required_product(&self) -> Option<&CIM_Product> {
        self.required_product.as_ref()
    }

    /// Sets the value of TypeOfDependency
    pub fn set_type_of_dependency(&mut self, value: u16) {
        self.type_of_dependency = Some(value);
    }

    /// Gets the value of TypeOfDependency
    pub fn get_type_of_dependency(&self) -> Option<&u16> {
        self.type_of_dependency.as_ref()
    }
}

