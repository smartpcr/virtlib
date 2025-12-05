// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PatchPackage struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PatchPackage {
    #[serde(flatten)]
    pub base: Win32_MSIResource,

/// 
    #[serde(rename = "PatchID")]
    pub patch_id: Option<String>,

/// 
    #[serde(rename = "ProductCode")]
    pub product_code: Option<String>,
}

impl Win32_PatchPackage {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_MSIResource::new(),
            patch_id: None,
            product_code: None,
        }
    }


    /// Sets the value of PatchID
    pub fn set_patch_id(&mut self, value: String) {
        self.patch_id = Some(value);
    }

    /// Gets the value of PatchID
    pub fn get_patch_id(&self) -> Option<&String> {
        self.patch_id.as_ref()
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

