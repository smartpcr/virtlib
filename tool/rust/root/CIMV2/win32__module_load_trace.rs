// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ModuleLoadTrace struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ModuleLoadTrace {
    #[serde(flatten)]
    pub base: Win32_ModuleTrace,

/// 
    #[serde(rename = "DefaultBase")]
    pub default_base: Option<u64>,

/// 
    #[serde(rename = "FileName")]
    pub file_name: Option<String>,

/// 
    #[serde(rename = "ImageBase")]
    pub image_base: Option<u64>,

/// 
    #[serde(rename = "ImageChecksum")]
    pub image_checksum: Option<u32>,

/// 
    #[serde(rename = "ImageSize")]
    pub image_size: Option<u64>,

/// 
    #[serde(rename = "ProcessID")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "TimeDateStamp")]
    pub time_date_stamp: Option<u32>,
}

impl Win32_ModuleLoadTrace {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_ModuleTrace::new(),
            default_base: None,
            file_name: None,
            image_base: None,
            image_checksum: None,
            image_size: None,
            process_id: None,
            time_date_stamp: None,
        }
    }


    /// Sets the value of DefaultBase
    pub fn set_default_base(&mut self, value: u64) {
        self.default_base = Some(value);
    }

    /// Gets the value of DefaultBase
    pub fn get_default_base(&self) -> Option<&u64> {
        self.default_base.as_ref()
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
    pub fn set_image_base(&mut self, value: u64) {
        self.image_base = Some(value);
    }

    /// Gets the value of ImageBase
    pub fn get_image_base(&self) -> Option<&u64> {
        self.image_base.as_ref()
    }

    /// Sets the value of ImageChecksum
    pub fn set_image_checksum(&mut self, value: u32) {
        self.image_checksum = Some(value);
    }

    /// Gets the value of ImageChecksum
    pub fn get_image_checksum(&self) -> Option<&u32> {
        self.image_checksum.as_ref()
    }

    /// Sets the value of ImageSize
    pub fn set_image_size(&mut self, value: u64) {
        self.image_size = Some(value);
    }

    /// Gets the value of ImageSize
    pub fn get_image_size(&self) -> Option<&u64> {
        self.image_size.as_ref()
    }

    /// Sets the value of ProcessID
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessID
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of TimeDateStamp
    pub fn set_time_date_stamp(&mut self, value: u32) {
        self.time_date_stamp = Some(value);
    }

    /// Gets the value of TimeDateStamp
    pub fn get_time_date_stamp(&self) -> Option<&u32> {
        self.time_date_stamp.as_ref()
    }
}

