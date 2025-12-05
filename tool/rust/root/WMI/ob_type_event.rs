// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ObTypeEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ObTypeEvent {
    #[serde(flatten)]
    pub base: ObTrace,

/// 
    #[serde(rename = "ObjectType")]
    pub object_type: Option<u16>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u16>,

/// 
    #[serde(rename = "TypeName")]
    pub type_name: Option<String>,
}

impl ObTypeEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: ObTrace::new(),
            object_type: None,
            reserved: None,
            type_name: None,
        }
    }


    /// Sets the value of ObjectType
    pub fn set_object_type(&mut self, value: u16) {
        self.object_type = Some(value);
    }

    /// Gets the value of ObjectType
    pub fn get_object_type(&self) -> Option<&u16> {
        self.object_type.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u16) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u16> {
        self.reserved.as_ref()
    }

    /// Sets the value of TypeName
    pub fn set_type_name(&mut self, value: String) {
        self.type_name = Some(value);
    }

    /// Gets the value of TypeName
    pub fn get_type_name(&self) -> Option<&String> {
        self.type_name.as_ref()
    }
}

