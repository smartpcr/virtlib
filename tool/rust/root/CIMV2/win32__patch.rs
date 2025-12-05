// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_Patch struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_Patch {
    #[serde(flatten)]
    pub base: Win32_MSIResource,

/// 
    #[serde(rename = "Attributes")]
    pub attributes: Option<u16>,

/// 
    #[serde(rename = "File")]
    pub file: Option<String>,

/// 
    #[serde(rename = "PatchSize")]
    pub patch_size: Option<u32>,

/// 
    #[serde(rename = "ProductCode")]
    pub product_code: Option<String>,

/// 
    #[serde(rename = "Sequence")]
    pub sequence: Option<i16>,
}

impl Win32_Patch {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_MSIResource::new(),
            attributes: None,
            file: None,
            patch_size: None,
            product_code: None,
            sequence: None,
        }
    }


    /// Sets the value of Attributes
    pub fn set_attributes(&mut self, value: u16) {
        self.attributes = Some(value);
    }

    /// Gets the value of Attributes
    pub fn get_attributes(&self) -> Option<&u16> {
        self.attributes.as_ref()
    }

    /// Sets the value of File
    pub fn set_file(&mut self, value: String) {
        self.file = Some(value);
    }

    /// Gets the value of File
    pub fn get_file(&self) -> Option<&String> {
        self.file.as_ref()
    }

    /// Sets the value of PatchSize
    pub fn set_patch_size(&mut self, value: u32) {
        self.patch_size = Some(value);
    }

    /// Gets the value of PatchSize
    pub fn get_patch_size(&self) -> Option<&u32> {
        self.patch_size.as_ref()
    }

    /// Sets the value of ProductCode
    pub fn set_product_code(&mut self, value: String) {
        self.product_code = Some(value);
    }

    /// Gets the value of ProductCode
    pub fn get_product_code(&self) -> Option<&String> {
        self.product_code.as_ref()
    }

    /// Sets the value of Sequence
    pub fn set_sequence(&mut self, value: i16) {
        self.sequence = Some(value);
    }

    /// Gets the value of Sequence
    pub fn get_sequence(&self) -> Option<&i16> {
        self.sequence.as_ref()
    }
}

