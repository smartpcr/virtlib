// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Process_V0_TypeGroup1 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Process_V0_TypeGroup1 {
    #[serde(flatten)]
    pub base: Process_V0,

/// 
    #[serde(rename = "ImageFileName")]
    pub image_file_name: Option<String>,

/// 
    #[serde(rename = "ParentId")]
    pub parent_id: Option<u32>,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "UserSID")]
    pub user_sid: Option<serde_json::Value>,
}

impl Process_V0_TypeGroup1 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Process_V0::new(),
            image_file_name: None,
            parent_id: None,
            process_id: None,
            user_sid: None,
        }
    }


    /// Sets the value of ImageFileName
    pub fn set_image_file_name(&mut self, value: String) {
        self.image_file_name = Some(value);
    }

    /// Gets the value of ImageFileName
    pub fn get_image_file_name(&self) -> Option<&String> {
        self.image_file_name.as_ref()
    }

    /// Sets the value of ParentId
    pub fn set_parent_id(&mut self, value: u32) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentId
    pub fn get_parent_id(&self) -> Option<&u32> {
        self.parent_id.as_ref()
    }

    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of UserSID
    pub fn set_user_sid(&mut self, value: serde_json::Value) {
        self.user_sid = Some(value);
    }

    /// Gets the value of UserSID
    pub fn get_user_sid(&self) -> Option<&serde_json::Value> {
        self.user_sid.as_ref()
    }
}

