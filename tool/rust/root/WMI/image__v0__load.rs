// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Image_V0_Load struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Image_V0_Load {
    #[serde(flatten)]
    pub base: Image_V0,

/// 
    #[serde(rename = "BaseAddress")]
    pub base_address: Option<u32>,

/// 
    #[serde(rename = "ImageFileName")]
    pub image_file_name: Option<String>,

/// 
    #[serde(rename = "ModuleSize")]
    pub module_size: Option<u32>,
}

impl Image_V0_Load {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Image_V0::new(),
            base_address: None,
            image_file_name: None,
            module_size: None,
        }
    }


    /// Sets the value of BaseAddress
    pub fn set_base_address(&mut self, value: u32) {
        self.base_address = Some(value);
    }

    /// Gets the value of BaseAddress
    pub fn get_base_address(&self) -> Option<&u32> {
        self.base_address.as_ref()
    }

    /// Sets the value of ImageFileName
    pub fn set_image_file_name(&mut self, value: String) {
        self.image_file_name = Some(value);
    }

    /// Gets the value of ImageFileName
    pub fn get_image_file_name(&self) -> Option<&String> {
        self.image_file_name.as_ref()
    }

    /// Sets the value of ModuleSize
    pub fn set_module_size(&mut self, value: u32) {
        self.module_size = Some(value);
    }

    /// Gets the value of ModuleSize
    pub fn get_module_size(&self) -> Option<&u32> {
        self.module_size.as_ref()
    }
}

