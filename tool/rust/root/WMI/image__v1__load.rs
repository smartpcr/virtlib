// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Image_V1_Load struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Image_V1_Load {
    #[serde(flatten)]
    pub base: Image_V1,

/// 
    #[serde(rename = "FileName")]
    pub file_name: Option<String>,

/// 
    #[serde(rename = "ImageBase")]
    pub image_base: Option<u32>,

/// 
    #[serde(rename = "ImageSize")]
    pub image_size: Option<u32>,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,
}

impl Image_V1_Load {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Image_V1::new(),
            file_name: None,
            image_base: None,
            image_size: None,
            process_id: None,
        }
    }


    /// Sets the value of FileName
    pub fn set_file_name(&mut self, value: String) {
        self.file_name = Some(value);
    }

    /// Gets the value of FileName
    pub fn get_file_name(&self) -> Option<&String> {
        self.file_name.as_ref()
    }

    /// Sets the value of ImageBase
    pub fn set_image_base(&mut self, value: u32) {
        self.image_base = Some(value);
    }

    /// Gets the value of ImageBase
    pub fn get_image_base(&self) -> Option<&u32> {
        self.image_base.as_ref()
    }

    /// Sets the value of ImageSize
    pub fn set_image_size(&mut self, value: u32) {
        self.image_size = Some(value);
    }

    /// Gets the value of ImageSize
    pub fn get_image_size(&self) -> Option<&u32> {
        self.image_size.as_ref()
    }

    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }
}

