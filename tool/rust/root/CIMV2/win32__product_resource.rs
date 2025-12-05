// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ProductResource struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ProductResource {

/// 
    #[serde(rename = "Product")]
    pub product: Option<Win32_Product>,

/// 
    #[serde(rename = "Resource")]
    pub resource: Option<Win32_MSIResource>,
}

impl Win32_ProductResource {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            product: None,
            resource: None,
        }
    }


    /// Sets the value of Product
    pub fn set_product(&mut self, value: Win32_Product) {
        self.product = Some(value);
    }

    /// Gets the value of Product
    pub fn get_product(&self) -> Option<&Win32_Product> {
        self.product.as_ref()
    }

    /// Sets the value of Resource
    pub fn set_resource(&mut self, value: Win32_MSIResource) {
        self.resource = Some(value);
    }

    /// Gets the value of Resource
    pub fn get_resource(&self) -> Option<&Win32_MSIResource> {
        self.resource.as_ref()
    }
}

