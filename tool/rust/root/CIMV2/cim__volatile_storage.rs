// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_VolatileStorage struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_VolatileStorage {
    #[serde(flatten)]
    pub base: CIM_Memory,

/// 
    #[serde(rename = "Cacheable")]
    pub cacheable: Option<bool>,

/// 
    #[serde(rename = "CacheType")]
    pub cache_type: Option<u16>,
}

impl CIM_VolatileStorage {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Memory::new(),
            cacheable: None,
            cache_type: None,
        }
    }


    /// Sets the value of Cacheable
    pub fn set_cacheable(&mut self, value: bool) {
        self.cacheable = Some(value);
    }

    /// Gets the value of Cacheable
    pub fn get_cacheable(&self) -> Option<&bool> {
        self.cacheable.as_ref()
    }

    /// Sets the value of CacheType
    pub fn set_cache_type(&mut self, value: u16) {
        self.cache_type = Some(value);
    }

    /// Gets the value of CacheType
    pub fn get_cache_type(&self) -> Option<&u16> {
        self.cache_type.as_ref()
    }
}

