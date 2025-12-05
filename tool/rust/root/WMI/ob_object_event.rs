// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ObObjectEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ObObjectEvent {
    #[serde(flatten)]
    pub base: ObTrace,

/// 
    #[serde(rename = "Object")]
    pub object: Option<u32>,

/// 
    #[serde(rename = "ObjectType")]
    pub object_type: Option<u16>,
}

impl ObObjectEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: ObTrace::new(),
            object: None,
            object_type: None,
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
}

