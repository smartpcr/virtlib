// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// FileIo_V2_ReadWrite struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileIo_V2_ReadWrite {
    #[serde(flatten)]
    pub base: FileIo_V2,

/// 
    #[serde(rename = "FileKey")]
    pub file_key: Option<u32>,

/// 
    #[serde(rename = "FileObject")]
    pub file_object: Option<u32>,

/// 
    #[serde(rename = "IoFlags")]
    pub io_flags: Option<u32>,

/// 
    #[serde(rename = "IoSize")]
    pub io_size: Option<u32>,

/// 
    #[serde(rename = "IrpPtr")]
    pub irp_ptr: Option<u32>,

/// 
    #[serde(rename = "Offset")]
    pub offset: Option<u64>,

/// 
    #[serde(rename = "TTID")]
    pub ttid: Option<u32>,
}

impl FileIo_V2_ReadWrite {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: FileIo_V2::new(),
            file_key: None,
            file_object: None,
            io_flags: None,
            io_size: None,
            irp_ptr: None,
            offset: None,
            ttid: None,
        }
    }


    /// Sets the value of FileKey
    pub fn set_file_key(&mut self, value: u32) {
        self.file_key = Some(value);
    }

    /// Gets the value of FileKey
    pub fn get_file_key(&self) -> Option<&u32> {
        self.file_key.as_ref()
    }

    /// Sets the value of FileObject
    pub fn set_file_object(&mut self, value: u32) {
        self.file_object = Some(value);
    }

    /// Gets the value of FileObject
    pub fn get_file_object(&self) -> Option<&u32> {
        self.file_object.as_ref()
    }

    /// Sets the value of IoFlags
    pub fn set_io_flags(&mut self, value: u32) {
        self.io_flags = Some(value);
    }

    /// Gets the value of IoFlags
    pub fn get_io_flags(&self) -> Option<&u32> {
        self.io_flags.as_ref()
    }

    /// Sets the value of IoSize
    pub fn set_io_size(&mut self, value: u32) {
        self.io_size = Some(value);
    }

    /// Gets the value of IoSize
    pub fn get_io_size(&self) -> Option<&u32> {
        self.io_size.as_ref()
    }

    /// Sets the value of IrpPtr
    pub fn set_irp_ptr(&mut self, value: u32) {
        self.irp_ptr = Some(value);
    }

    /// Gets the value of IrpPtr
    pub fn get_irp_ptr(&self) -> Option<&u32> {
        self.irp_ptr.as_ref()
    }

    /// Sets the value of Offset
    pub fn set_offset(&mut self, value: u64) {
        self.offset = Some(value);
    }

    /// Gets the value of Offset
    pub fn get_offset(&self) -> Option<&u64> {
        self.offset.as_ref()
    }

    /// Sets the value of TTID
    pub fn set_ttid(&mut self, value: u32) {
        self.ttid = Some(value);
    }

    /// Gets the value of TTID
    pub fn get_ttid(&self) -> Option<&u32> {
        self.ttid.as_ref()
    }
}

