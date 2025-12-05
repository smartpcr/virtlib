// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// FileIo_Create struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileIo_Create {
    #[serde(flatten)]
    pub base: FileIo,

/// 
    #[serde(rename = "CreateOptions")]
    pub create_options: Option<u32>,

/// 
    #[serde(rename = "FileAttributes")]
    pub file_attributes: Option<u32>,

/// 
    #[serde(rename = "FileObject")]
    pub file_object: Option<u32>,

/// 
    #[serde(rename = "IrpPtr")]
    pub irp_ptr: Option<u32>,

/// 
    #[serde(rename = "OpenPath")]
    pub open_path: Option<String>,

/// 
    #[serde(rename = "ShareAccess")]
    pub share_access: Option<u32>,

/// 
    #[serde(rename = "TTID")]
    pub ttid: Option<u32>,
}

impl FileIo_Create {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: FileIo::new(),
            create_options: None,
            file_attributes: None,
            file_object: None,
            irp_ptr: None,
            open_path: None,
            share_access: None,
            ttid: None,
        }
    }


    /// Sets the value of CreateOptions
    pub fn set_create_options(&mut self, value: u32) {
        self.create_options = Some(value);
    }

    /// Gets the value of CreateOptions
    pub fn get_create_options(&self) -> Option<&u32> {
        self.create_options.as_ref()
    }

    /// Sets the value of FileAttributes
    pub fn set_file_attributes(&mut self, value: u32) {
        self.file_attributes = Some(value);
    }

    /// Gets the value of FileAttributes
    pub fn get_file_attributes(&self) -> Option<&u32> {
        self.file_attributes.as_ref()
    }

    /// Sets the value of FileObject
    pub fn set_file_object(&mut self, value: u32) {
        self.file_object = Some(value);
    }

    /// Gets the value of FileObject
    pub fn get_file_object(&self) -> Option<&u32> {
        self.file_object.as_ref()
    }

    /// Sets the value of IrpPtr
    pub fn set_irp_ptr(&mut self, value: u32) {
        self.irp_ptr = Some(value);
    }

    /// Gets the value of IrpPtr
    pub fn get_irp_ptr(&self) -> Option<&u32> {
        self.irp_ptr.as_ref()
    }

    /// Sets the value of OpenPath
    pub fn set_open_path(&mut self, value: String) {
        self.open_path = Some(value);
    }

    /// Gets the value of OpenPath
    pub fn get_open_path(&self) -> Option<&String> {
        self.open_path.as_ref()
    }

    /// Sets the value of ShareAccess
    pub fn set_share_access(&mut self, value: u32) {
        self.share_access = Some(value);
    }

    /// Gets the value of ShareAccess
    pub fn get_share_access(&self) -> Option<&u32> {
        self.share_access.as_ref()
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

