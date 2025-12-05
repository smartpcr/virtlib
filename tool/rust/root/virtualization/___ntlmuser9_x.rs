// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __NTLMUser9X struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __NTLMUser9X {
    #[serde(flatten)]
    pub base: __SecurityRelatedClass,

/// 
    #[serde(rename = "Authority")]
    pub authority: Option<String>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<i32>,

/// 
    #[serde(rename = "Mask")]
    pub mask: Option<i32>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<i32>,
}

impl __NTLMUser9X {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __SecurityRelatedClass::new(),
            authority: None,
            flags: None,
            mask: None,
            name: None,
            type: None,
        }
    }


    /// Sets the value of Authority
    pub fn set_authority(&mut self, value: String) {
        self.authority = Some(value);
    }

    /// Gets the value of Authority
    pub fn get_authority(&self) -> Option<&String> {
        self.authority.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: i32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&i32> {
        self.flags.as_ref()
    }

    /// Sets the value of Mask
    pub fn set_mask(&mut self, value: i32) {
        self.mask = Some(value);
    }

    /// Gets the value of Mask
    pub fn get_mask(&self) -> Option<&i32> {
        self.mask.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: i32) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&i32> {
        self.type.as_ref()
    }
}

