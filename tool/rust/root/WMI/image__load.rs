// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Image_Load struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Image_Load {
    #[serde(flatten)]
    pub base: Image,

/// 
    #[serde(rename = "DefaultBase")]
    pub default_base: Option<u32>,

/// 
    #[serde(rename = "FileName")]
    pub file_name: Option<String>,

/// 
    #[serde(rename = "ImageBase")]
    pub image_base: Option<u32>,

/// 
    #[serde(rename = "ImageChecksum")]
    pub image_checksum: Option<u32>,

/// 
    #[serde(rename = "ImageSize")]
    pub image_size: Option<u32>,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "Reserved0")]
    pub reserved0: Option<u16>,

/// 
    #[serde(rename = "Reserved1")]
    pub reserved1: Option<u32>,

/// 
    #[serde(rename = "Reserved2")]
    pub reserved2: Option<u32>,

/// 
    #[serde(rename = "Reserved3")]
    pub reserved3: Option<u32>,

/// 
    #[serde(rename = "Reserved4")]
    pub reserved4: Option<u32>,

/// 
    #[serde(rename = "SignatureLevel")]
    pub signature_level: Option<u8>,

/// 
    #[serde(rename = "SignatureType")]
    pub signature_type: Option<u8>,

/// 
    #[serde(rename = "TimeDateStamp")]
    pub time_date_stamp: Option<u32>,
}

impl Image_Load {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Image::new(),
            default_base: None,
            file_name: None,
            image_base: None,
            image_checksum: None,
            image_size: None,
            process_id: None,
            reserved0: None,
            reserved1: None,
            reserved2: None,
            reserved3: None,
            reserved4: None,
            signature_level: None,
            signature_type: None,
            time_date_stamp: None,
        }
    }


    /// Sets the value of DefaultBase
    pub fn set_default_base(&mut self, value: u32) {
        self.default_base = Some(value);
    }

    /// Gets the value of DefaultBase
    pub fn get_default_base(&self) -> Option<&u32> {
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
    pub fn set_image_base(&mut self, value: u32) {
        self.image_base = Some(value);
    }

    /// Gets the value of ImageBase
    pub fn get_image_base(&self) -> Option<&u32> {
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

    /// Sets the value of Reserved0
    pub fn set_reserved0(&mut self, value: u16) {
        self.reserved0 = Some(value);
    }

    /// Gets the value of Reserved0
    pub fn get_reserved0(&self) -> Option<&u16> {
        self.reserved0.as_ref()
    }

    /// Sets the value of Reserved1
    pub fn set_reserved1(&mut self, value: u32) {
        self.reserved1 = Some(value);
    }

    /// Gets the value of Reserved1
    pub fn get_reserved1(&self) -> Option<&u32> {
        self.reserved1.as_ref()
    }

    /// Sets the value of Reserved2
    pub fn set_reserved2(&mut self, value: u32) {
        self.reserved2 = Some(value);
    }

    /// Gets the value of Reserved2
    pub fn get_reserved2(&self) -> Option<&u32> {
        self.reserved2.as_ref()
    }

    /// Sets the value of Reserved3
    pub fn set_reserved3(&mut self, value: u32) {
        self.reserved3 = Some(value);
    }

    /// Gets the value of Reserved3
    pub fn get_reserved3(&self) -> Option<&u32> {
        self.reserved3.as_ref()
    }

    /// Sets the value of Reserved4
    pub fn set_reserved4(&mut self, value: u32) {
        self.reserved4 = Some(value);
    }

    /// Gets the value of Reserved4
    pub fn get_reserved4(&self) -> Option<&u32> {
        self.reserved4.as_ref()
    }

    /// Sets the value of SignatureLevel
    pub fn set_signature_level(&mut self, value: u8) {
        self.signature_level = Some(value);
    }

    /// Gets the value of SignatureLevel
    pub fn get_signature_level(&self) -> Option<&u8> {
        self.signature_level.as_ref()
    }

    /// Sets the value of SignatureType
    pub fn set_signature_type(&mut self, value: u8) {
        self.signature_type = Some(value);
    }

    /// Gets the value of SignatureType
    pub fn get_signature_type(&self) -> Option<&u8> {
        self.signature_type.as_ref()
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

