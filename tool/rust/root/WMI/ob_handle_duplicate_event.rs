// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ObHandleDuplicateEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ObHandleDuplicateEvent {
    #[serde(flatten)]
    pub base: ObTrace,

/// 
    #[serde(rename = "Object")]
    pub object: Option<u32>,

/// 
    #[serde(rename = "ObjectType")]
    pub object_type: Option<u16>,

/// 
    #[serde(rename = "SourceHandle")]
    pub source_handle: Option<u32>,

/// 
    #[serde(rename = "TargetHandle")]
    pub target_handle: Option<u32>,

/// 
    #[serde(rename = "TargetProcessId")]
    pub target_process_id: Option<u32>,
}

impl ObHandleDuplicateEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: ObTrace::new(),
            object: None,
            object_type: None,
            source_handle: None,
            target_handle: None,
            target_process_id: None,
        }
    }


    /// Sets the value of Object
    pub fn set_object(&mut self, value: u32) {
        self.object = Some(value);
    }

    /// Gets the value of Object
    pub fn get_object(&self) -> Option<&u32> {
        self.object.as_ref()
    }

    /// Sets the value of ObjectType
    pub fn set_object_type(&mut self, value: u16) {
        self.object_type = Some(value);
    }

    /// Gets the value of ObjectType
    pub fn get_object_type(&self) -> Option<&u16> {
        self.object_type.as_ref()
    }

    /// Sets the value of SourceHandle
    pub fn set_source_handle(&mut self, value: u32) {
        self.source_handle = Some(value);
    }

    /// Gets the value of SourceHandle
    pub fn get_source_handle(&self) -> Option<&u32> {
        self.source_handle.as_ref()
    }

    /// Sets the value of TargetHandle
    pub fn set_target_handle(&mut self, value: u32) {
        self.target_handle = Some(value);
    }

    /// Gets the value of TargetHandle
    pub fn get_target_handle(&self) -> Option<&u32> {
        self.target_handle.as_ref()
    }

    /// Sets the value of TargetProcessId
    pub fn set_target_process_id(&mut self, value: u32) {
        self.target_process_id = Some(value);
    }

    /// Gets the value of TargetProcessId
    pub fn get_target_process_id(&self) -> Option<&u32> {
        self.target_process_id.as_ref()
    }
}

