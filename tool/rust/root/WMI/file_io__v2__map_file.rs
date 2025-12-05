// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// FileIo_V2_MapFile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileIo_V2_MapFile {
    #[serde(flatten)]
    pub base: FileIo_V2,

/// 
    #[serde(rename = "FileObject")]
    pub file_object: Option<u32>,

/// 
    #[serde(rename = "MiscInfo")]
    pub misc_info: Option<u64>,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "ViewBase")]
    pub view_base: Option<u32>,

/// 
    #[serde(rename = "ViewSize")]
    pub view_size: Option<serde_json::Value>,
}

impl FileIo_V2_MapFile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: FileIo_V2::new(),
            file_object: None,
            misc_info: None,
            process_id: None,
            view_base: None,
            view_size: None,
        }
    }


    /// Sets the value of FileObject
    pub fn set_file_object(&mut self, value: u32) {
        self.file_object = Some(value);
    }

    /// Gets the value of FileObject
    pub fn get_file_object(&self) -> Option<&u32> {
        self.file_object.as_ref()
    }

    /// Sets the value of MiscInfo
    pub fn set_misc_info(&mut self, value: u64) {
        self.misc_info = Some(value);
    }

    /// Gets the value of MiscInfo
    pub fn get_misc_info(&self) -> Option<&u64> {
        self.misc_info.as_ref()
    }

    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of ViewBase
    pub fn set_view_base(&mut self, value: u32) {
        self.view_base = Some(value);
    }

    /// Gets the value of ViewBase
    pub fn get_view_base(&self) -> Option<&u32> {
        self.view_base.as_ref()
    }

    /// Sets the value of ViewSize
    pub fn set_view_size(&mut self, value: serde_json::Value) {
        self.view_size = Some(value);
    }

    /// Gets the value of ViewSize
    pub fn get_view_size(&self) -> Option<&serde_json::Value> {
        self.view_size.as_ref()
    }
}

