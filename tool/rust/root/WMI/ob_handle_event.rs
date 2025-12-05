// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ObHandleEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ObHandleEvent {
    #[serde(flatten)]
    pub base: ObTrace,

/// 
    #[serde(rename = "Handle")]
    pub handle: Option<u32>,

/// 
    #[serde(rename = "Object")]
    pub object: Option<u32>,

/// 
    #[serde(rename = "ObjectName")]
    pub object_name: Option<String>,

/// 
    #[serde(rename = "ObjectType")]
    pub object_type: Option<u16>,
}

impl ObHandleEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: ObTrace::new(),
            handle: None,
            object: None,
            object_name: None,
            object_type: None,
        }
    }


    /// Sets the value of Handle
    pub fn set_handle(&mut self, value: u32) {
        self.handle = Some(value);
    }

    /// Gets the value of Handle
    pub fn get_handle(&self) -> Option<&u32> {
        self.handle.as_ref()
    }

    /// Sets the value of Object
    pub fn set_object(&mut self, value: u32) {
        self.object = Some(value);
    }

    /// Gets the value of Object
    pub fn get_object(&self) -> Option<&u32> {
        self.object.as_ref()
    }

    /// Sets the value of ObjectName
    pub fn set_object_name(&mut self, value: String) {
        self.object_name = Some(value);
    }

    /// Gets the value of ObjectName
    pub fn get_object_name(&self) -> Option<&String> {
        self.object_name.as_ref()
    }

    /// Sets the value of ObjectType
    pub fn set_object_type(&mut self, value: u16) {
        self.object_type = Some(value);
    }

    /// Gets the value of ObjectType
    pub fn get_object_type(&self) -> Option<&u16> {
        self.object_type.as_ref()
    }
}

