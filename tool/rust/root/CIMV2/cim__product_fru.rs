// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ProductFRU struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ProductFRU {

/// 
    #[serde(rename = "FRU")]
    pub fru: Option<CIM_FRU>,

/// 
    #[serde(rename = "Product")]
    pub product: Option<CIM_Product>,
}

impl CIM_ProductFRU {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            fru: None,
            product: None,
        }
    }


    /// Sets the value of FRU
    pub fn set_fru(&mut self, value: CIM_FRU) {
        self.fru = Some(value);
    }

    /// Gets the value of FRU
    pub fn get_fru(&self) -> Option<&CIM_FRU> {
        self.fru.as_ref()
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

