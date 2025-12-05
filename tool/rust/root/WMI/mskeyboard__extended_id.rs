// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSKeyboard_ExtendedID struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSKeyboard_ExtendedID {
    #[serde(flatten)]
    pub base: MSKeyboard,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "Subtype")]
    pub subtype: Option<u32>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u32>,
}

impl MSKeyboard_ExtendedID {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSKeyboard::new(),
            active: None,
            instance_name: None,
            subtype: None,
            type: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of Subtype
    pub fn set_subtype(&mut self, value: u32) {
        self.subtype = Some(value);
    }

    /// Gets the value of Subtype
    pub fn get_subtype(&self) -> Option<&u32> {
        self.subtype.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u32) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u32> {
        self.type.as_ref()
    }
}

