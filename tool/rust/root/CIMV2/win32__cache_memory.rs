// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_CacheMemory struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_CacheMemory {
    #[serde(flatten)]
    pub base: CIM_CacheMemory,

/// 
    #[serde(rename = "CacheSpeed")]
    pub cache_speed: Option<u32>,

/// 
    #[serde(rename = "CurrentSRAM")]
    pub current_sram: Vec<u16>,

/// 
    #[serde(rename = "ErrorCorrectType")]
    pub error_correct_type: Option<u16>,

/// 
    #[serde(rename = "InstalledSize")]
    pub installed_size: Option<u32>,

/// 
    #[serde(rename = "Location")]
    pub location: Option<u16>,

/// 
    #[serde(rename = "MaxCacheSize")]
    pub max_cache_size: Option<u32>,

/// 
    #[serde(rename = "SupportedSRAM")]
    pub supported_sram: Vec<u16>,
}

impl Win32_CacheMemory {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_CacheMemory::new(),
            cache_speed: None,
            current_sram: Vec::new(),
            error_correct_type: None,
            installed_size: None,
            location: None,
            max_cache_size: None,
            supported_sram: Vec::new(),
        }
    }


    /// Sets the value of CacheSpeed
    pub fn set_cache_speed(&mut self, value: u32) {
        self.cache_speed = Some(value);
    }

    /// Gets the value of CacheSpeed
    pub fn get_cache_speed(&self) -> Option<&u32> {
        self.cache_speed.as_ref()
    }

    /// Sets the value of CurrentSRAM
    pub fn set_current_sram(&mut self, value: Vec<u16>) {
        self.current_sram = value;
    }

    /// Gets the value of CurrentSRAM
    pub fn get_current_sram(&self) -> &Vec<u16> {
        &self.current_sram
    }

    /// Sets the value of ErrorCorrectType
    pub fn set_error_correct_type(&mut self, value: u16) {
        self.error_correct_type = Some(value);
    }

    /// Gets the value of ErrorCorrectType
    pub fn get_error_correct_type(&self) -> Option<&u16> {
        self.error_correct_type.as_ref()
    }

    /// Sets the value of InstalledSize
    pub fn set_installed_size(&mut self, value: u32) {
        self.installed_size = Some(value);
    }

    /// Gets the value of InstalledSize
    pub fn get_installed_size(&self) -> Option<&u32> {
        self.installed_size.as_ref()
    }

    /// Sets the value of Location
    pub fn set_location(&mut self, value: u16) {
        self.location = Some(value);
    }

    /// Gets the value of Location
    pub fn get_location(&self) -> Option<&u16> {
        self.location.as_ref()
    }

    /// Sets the value of MaxCacheSize
    pub fn set_max_cache_size(&mut self, value: u32) {
        self.max_cache_size = Some(value);
    }

    /// Gets the value of MaxCacheSize
    pub fn get_max_cache_size(&self) -> Option<&u32> {
        self.max_cache_size.as_ref()
    }

    /// Sets the value of SupportedSRAM
    pub fn set_supported_sram(&mut self, value: Vec<u16>) {
        self.supported_sram = value;
    }

    /// Gets the value of SupportedSRAM
    pub fn get_supported_sram(&self) -> &Vec<u16> {
        &self.supported_sram
    }
}

