// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PageFileUsage struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PageFileUsage {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "AllocatedBaseSize")]
    pub allocated_base_size: Option<u32>,

/// 
    #[serde(rename = "CurrentUsage")]
    pub current_usage: Option<u32>,

/// 
    #[serde(rename = "PeakUsage")]
    pub peak_usage: Option<u32>,

/// 
    #[serde(rename = "TempPageFile")]
    pub temp_page_file: Option<bool>,
}

impl Win32_PageFileUsage {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            allocated_base_size: None,
            current_usage: None,
            peak_usage: None,
            temp_page_file: None,
        }
    }


    /// Sets the value of AllocatedBaseSize
    pub fn set_allocated_base_size(&mut self, value: u32) {
        self.allocated_base_size = Some(value);
    }

    /// Gets the value of AllocatedBaseSize
    pub fn get_allocated_base_size(&self) -> Option<&u32> {
        self.allocated_base_size.as_ref()
    }

    /// Sets the value of CurrentUsage
    pub fn set_current_usage(&mut self, value: u32) {
        self.current_usage = Some(value);
    }

    /// Gets the value of CurrentUsage
    pub fn get_current_usage(&self) -> Option<&u32> {
        self.current_usage.as_ref()
    }

    /// Sets the value of PeakUsage
    pub fn set_peak_usage(&mut self, value: u32) {
        self.peak_usage = Some(value);
    }

    /// Gets the value of PeakUsage
    pub fn get_peak_usage(&self) -> Option<&u32> {
        self.peak_usage.as_ref()
    }

    /// Sets the value of TempPageFile
    pub fn set_temp_page_file(&mut self, value: bool) {
        self.temp_page_file = Some(value);
    }

    /// Gets the value of TempPageFile
    pub fn get_temp_page_file(&self) -> Option<&bool> {
        self.temp_page_file.as_ref()
    }
}

