// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_WmiHDSplitCurrentConfig struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_WmiHDSplitCurrentConfig {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "BackfillSize")]
    pub backfill_size: Option<u32>,

/// 
    #[serde(rename = "CurrentCapabilities")]
    pub current_capabilities: Option<u32>,

/// 
    #[serde(rename = "HardwareCapabilities")]
    pub hardware_capabilities: Option<u32>,

/// 
    #[serde(rename = "HDSplitCombineFlags")]
    pub hdsplit_combine_flags: Option<u32>,

/// 
    #[serde(rename = "HDSplitFlags")]
    pub hdsplit_flags: Option<u32>,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,

/// 
    #[serde(rename = "MaxHeaderSize")]
    pub max_header_size: Option<u32>,
}

impl MSNdis_WmiHDSplitCurrentConfig {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            backfill_size: None,
            current_capabilities: None,
            hardware_capabilities: None,
            hdsplit_combine_flags: None,
            hdsplit_flags: None,
            header: None,
            max_header_size: None,
        }
    }


    /// Sets the value of BackfillSize
    pub fn set_backfill_size(&mut self, value: u32) {
        self.backfill_size = Some(value);
    }

    /// Gets the value of BackfillSize
    pub fn get_backfill_size(&self) -> Option<&u32> {
        self.backfill_size.as_ref()
    }

    /// Sets the value of CurrentCapabilities
    pub fn set_current_capabilities(&mut self, value: u32) {
        self.current_capabilities = Some(value);
    }

    /// Gets the value of CurrentCapabilities
    pub fn get_current_capabilities(&self) -> Option<&u32> {
        self.current_capabilities.as_ref()
    }

    /// Sets the value of HardwareCapabilities
    pub fn set_hardware_capabilities(&mut self, value: u32) {
        self.hardware_capabilities = Some(value);
    }

    /// Gets the value of HardwareCapabilities
    pub fn get_hardware_capabilities(&self) -> Option<&u32> {
        self.hardware_capabilities.as_ref()
    }

    /// Sets the value of HDSplitCombineFlags
    pub fn set_hdsplit_combine_flags(&mut self, value: u32) {
        self.hdsplit_combine_flags = Some(value);
    }

    /// Gets the value of HDSplitCombineFlags
    pub fn get_hdsplit_combine_flags(&self) -> Option<&u32> {
        self.hdsplit_combine_flags.as_ref()
    }

    /// Sets the value of HDSplitFlags
    pub fn set_hdsplit_flags(&mut self, value: u32) {
        self.hdsplit_flags = Some(value);
    }

    /// Gets the value of HDSplitFlags
    pub fn get_hdsplit_flags(&self) -> Option<&u32> {
        self.hdsplit_flags.as_ref()
    }

    /// Sets the value of Header
    pub fn set_header(&mut self, value: MSNdis_ObjectHeader) {
        self.header = Some(value);
    }

    /// Gets the value of Header
    pub fn get_header(&self) -> Option<&MSNdis_ObjectHeader> {
        self.header.as_ref()
    }

    /// Sets the value of MaxHeaderSize
    pub fn set_max_header_size(&mut self, value: u32) {
        self.max_header_size = Some(value);
    }

    /// Gets the value of MaxHeaderSize
    pub fn get_max_header_size(&self) -> Option<&u32> {
        self.max_header_size.as_ref()
    }
}

