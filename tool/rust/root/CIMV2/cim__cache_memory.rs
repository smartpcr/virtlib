// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_CacheMemory struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_CacheMemory {
    #[serde(flatten)]
    pub base: CIM_Memory,

/// 
    #[serde(rename = "Associativity")]
    pub associativity: Option<u16>,

/// 
    #[serde(rename = "CacheType")]
    pub cache_type: Option<u16>,

/// 
    #[serde(rename = "FlushTimer")]
    pub flush_timer: Option<u32>,

/// 
    #[serde(rename = "Level")]
    pub level: Option<u16>,

/// 
    #[serde(rename = "LineSize")]
    pub line_size: Option<u32>,

/// 
    #[serde(rename = "ReadPolicy")]
    pub read_policy: Option<u16>,

/// 
    #[serde(rename = "ReplacementPolicy")]
    pub replacement_policy: Option<u16>,

/// 
    #[serde(rename = "WritePolicy")]
    pub write_policy: Option<u16>,
}

impl CIM_CacheMemory {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Memory::new(),
            associativity: None,
            cache_type: None,
            flush_timer: None,
            level: None,
            line_size: None,
            read_policy: None,
            replacement_policy: None,
            write_policy: None,
        }
    }


    /// Sets the value of Associativity
    pub fn set_associativity(&mut self, value: u16) {
        self.associativity = Some(value);
    }

    /// Gets the value of Associativity
    pub fn get_associativity(&self) -> Option<&u16> {
        self.associativity.as_ref()
    }

    /// Sets the value of CacheType
    pub fn set_cache_type(&mut self, value: u16) {
        self.cache_type = Some(value);
    }

    /// Gets the value of CacheType
    pub fn get_cache_type(&self) -> Option<&u16> {
        self.cache_type.as_ref()
    }

    /// Sets the value of FlushTimer
    pub fn set_flush_timer(&mut self, value: u32) {
        self.flush_timer = Some(value);
    }

    /// Gets the value of FlushTimer
    pub fn get_flush_timer(&self) -> Option<&u32> {
        self.flush_timer.as_ref()
    }

    /// Sets the value of Level
    pub fn set_level(&mut self, value: u16) {
        self.level = Some(value);
    }

    /// Gets the value of Level
    pub fn get_level(&self) -> Option<&u16> {
        self.level.as_ref()
    }

    /// Sets the value of LineSize
    pub fn set_line_size(&mut self, value: u32) {
        self.line_size = Some(value);
    }

    /// Gets the value of LineSize
    pub fn get_line_size(&self) -> Option<&u32> {
        self.line_size.as_ref()
    }

    /// Sets the value of ReadPolicy
    pub fn set_read_policy(&mut self, value: u16) {
        self.read_policy = Some(value);
    }

    /// Gets the value of ReadPolicy
    pub fn get_read_policy(&self) -> Option<&u16> {
        self.read_policy.as_ref()
    }

    /// Sets the value of ReplacementPolicy
    pub fn set_replacement_policy(&mut self, value: u16) {
        self.replacement_policy = Some(value);
    }

    /// Gets the value of ReplacementPolicy
    pub fn get_replacement_policy(&self) -> Option<&u16> {
        self.replacement_policy.as_ref()
    }

    /// Sets the value of WritePolicy
    pub fn set_write_policy(&mut self, value: u16) {
        self.write_policy = Some(value);
    }

    /// Gets the value of WritePolicy
    pub fn get_write_policy(&self) -> Option<&u16> {
        self.write_policy.as_ref()
    }
}

