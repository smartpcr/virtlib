// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Process_V2_TypeGroup4 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Process_V2_TypeGroup4 {
    #[serde(flatten)]
    pub base: Process_V2,

/// 
    #[serde(rename = "Count")]
    pub count: Option<u32>,

/// 
    #[serde(rename = "Object")]
    pub object: Option<u32>,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "Tag")]
    pub tag: Option<u32>,
}

impl Process_V2_TypeGroup4 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Process_V2::new(),
            count: None,
            object: None,
            process_id: None,
            tag: None,
        }
    }


    /// Sets the value of Count
    pub fn set_count(&mut self, value: u32) {
        self.count = Some(value);
    }

    /// Gets the value of Count
    pub fn get_count(&self) -> Option<&u32> {
        self.count.as_ref()
    }

    /// Sets the value of Object
    pub fn set_object(&mut self, value: u32) {
        self.object = Some(value);
    }

    /// Gets the value of Object
    pub fn get_object(&self) -> Option<&u32> {
        self.object.as_ref()
    }

    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of Tag
    pub fn set_tag(&mut self, value: u32) {
        self.tag = Some(value);
    }

    /// Gets the value of Tag
    pub fn get_tag(&self) -> Option<&u32> {
        self.tag.as_ref()
    }
}

