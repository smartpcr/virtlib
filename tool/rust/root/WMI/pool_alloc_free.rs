// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PoolAllocFree struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PoolAllocFree {
    #[serde(flatten)]
    pub base: PoolTrace,

/// 
    #[serde(rename = "Entry")]
    pub entry: Option<u32>,

/// 
    #[serde(rename = "NumberOfBytes")]
    pub number_of_bytes: Option<serde_json::Value>,

/// 
    #[serde(rename = "Tag")]
    pub tag: Option<u32>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u32>,
}

impl PoolAllocFree {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PoolTrace::new(),
            entry: None,
            number_of_bytes: None,
            tag: None,
            type: None,
        }
    }


    /// Sets the value of Entry
    pub fn set_entry(&mut self, value: u32) {
        self.entry = Some(value);
    }

    /// Gets the value of Entry
    pub fn get_entry(&self) -> Option<&u32> {
        self.entry.as_ref()
    }

    /// Sets the value of NumberOfBytes
    pub fn set_number_of_bytes(&mut self, value: serde_json::Value) {
        self.number_of_bytes = Some(value);
    }

    /// Gets the value of NumberOfBytes
    pub fn get_number_of_bytes(&self) -> Option<&serde_json::Value> {
        self.number_of_bytes.as_ref()
    }

    /// Sets the value of Tag
    pub fn set_tag(&mut self, value: u32) {
        self.tag = Some(value);
    }

    /// Gets the value of Tag
    pub fn get_tag(&self) -> Option<&u32> {
        self.tag.as_ref()
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

