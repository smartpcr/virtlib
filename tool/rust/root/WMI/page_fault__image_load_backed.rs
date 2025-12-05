// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PageFault_ImageLoadBacked struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PageFault_ImageLoadBacked {
    #[serde(flatten)]
    pub base: PageFault_V2,

/// 
    #[serde(rename = "DeviceChar")]
    pub device_char: Option<u32>,

/// 
    #[serde(rename = "FileChar")]
    pub file_char: Option<u16>,

/// 
    #[serde(rename = "FileObject")]
    pub file_object: Option<u32>,

/// 
    #[serde(rename = "LoadFlags")]
    pub load_flags: Option<u16>,
}

impl PageFault_ImageLoadBacked {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PageFault_V2::new(),
            device_char: None,
            file_char: None,
            file_object: None,
            load_flags: None,
        }
    }


    /// Sets the value of DeviceChar
    pub fn set_device_char(&mut self, value: u32) {
        self.device_char = Some(value);
    }

    /// Gets the value of DeviceChar
    pub fn get_device_char(&self) -> Option<&u32> {
        self.device_char.as_ref()
    }

    /// Sets the value of FileChar
    pub fn set_file_char(&mut self, value: u16) {
        self.file_char = Some(value);
    }

    /// Gets the value of FileChar
    pub fn get_file_char(&self) -> Option<&u16> {
        self.file_char.as_ref()
    }

    /// Sets the value of FileObject
    pub fn set_file_object(&mut self, value: u32) {
        self.file_object = Some(value);
    }

    /// Gets the value of FileObject
    pub fn get_file_object(&self) -> Option<&u32> {
        self.file_object.as_ref()
    }

    /// Sets the value of LoadFlags
    pub fn set_load_flags(&mut self, value: u16) {
        self.load_flags = Some(value);
    }

    /// Gets the value of LoadFlags
    pub fn get_load_flags(&self) -> Option<&u16> {
        self.load_flags.as_ref()
    }
}

