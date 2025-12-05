// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// FileIo_PathOperation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileIo_PathOperation {
    #[serde(flatten)]
    pub base: FileIo,

/// 
    #[serde(rename = "ExtraInfo")]
    pub extra_info: Option<u32>,

/// 
    #[serde(rename = "FileKey")]
    pub file_key: Option<u32>,

/// 
    #[serde(rename = "FileName")]
    pub file_name: Option<String>,

/// 
    #[serde(rename = "FileObject")]
    pub file_object: Option<u32>,

/// 
    #[serde(rename = "InfoClass")]
    pub info_class: Option<u32>,

/// 
    #[serde(rename = "IrpPtr")]
    pub irp_ptr: Option<u32>,

/// 
    #[serde(rename = "TTID")]
    pub ttid: Option<u32>,
}

impl FileIo_PathOperation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: FileIo::new(),
            extra_info: None,
            file_key: None,
            file_name: None,
            file_object: None,
            info_class: None,
            irp_ptr: None,
            ttid: None,
        }
    }


    /// Sets the value of ExtraInfo
    pub fn set_extra_info(&mut self, value: u32) {
        self.extra_info = Some(value);
    }

    /// Gets the value of ExtraInfo
    pub fn get_extra_info(&self) -> Option<&u32> {
        self.extra_info.as_ref()
    }

    /// Sets the value of FileKey
    pub fn set_file_key(&mut self, value: u32) {
        self.file_key = Some(value);
    }

    /// Gets the value of FileKey
    pub fn get_file_key(&self) -> Option<&u32> {
        self.file_key.as_ref()
    }

    /// Sets the value of FileName
    pub fn set_file_name(&mut self, value: String) {
        self.file_name = Some(value);
    }

    /// Gets the value of FileName
    pub fn get_file_name(&self) -> Option<&String> {
        self.file_name.as_ref()
    }

    /// Sets the value of FileObject
    pub fn set_file_object(&mut self, value: u32) {
        self.file_object = Some(value);
    }

    /// Gets the value of FileObject
    pub fn get_file_object(&self) -> Option<&u32> {
        self.file_object.as_ref()
    }

    /// Sets the value of InfoClass
    pub fn set_info_class(&mut self, value: u32) {
        self.info_class = Some(value);
    }

    /// Gets the value of InfoClass
    pub fn get_info_class(&self) -> Option<&u32> {
        self.info_class.as_ref()
    }

    /// Sets the value of IrpPtr
    pub fn set_irp_ptr(&mut self, value: u32) {
        self.irp_ptr = Some(value);
    }

    /// Gets the value of IrpPtr
    pub fn get_irp_ptr(&self) -> Option<&u32> {
        self.irp_ptr.as_ref()
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

