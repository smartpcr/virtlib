// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// KernelImageBase struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KernelImageBase {
    #[serde(flatten)]
    pub base: Image_V2,

/// 
    #[serde(rename = "ImageBase")]
    pub image_base: Option<u32>,
}

impl KernelImageBase {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Image_V2::new(),
            image_base: None,
        }
    }


    /// Sets the value of ImageBase
    pub fn set_image_base(&mut self, value: u32) {
        self.image_base = Some(value);
    }

    /// Gets the value of ImageBase
    pub fn get_image_base(&self) -> Option<&u32> {
        self.image_base.as_ref()
    }
}

