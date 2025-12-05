// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_VideoController struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_VideoController {
    #[serde(flatten)]
    pub base: CIM_Controller,

/// 
    #[serde(rename = "AcceleratorCapabilities")]
    pub accelerator_capabilities: Vec<u16>,

/// 
    #[serde(rename = "CapabilityDescriptions")]
    pub capability_descriptions: Vec<String>,

/// 
    #[serde(rename = "CurrentBitsPerPixel")]
    pub current_bits_per_pixel: Option<u32>,

/// 
    #[serde(rename = "CurrentHorizontalResolution")]
    pub current_horizontal_resolution: Option<u32>,

/// 
    #[serde(rename = "CurrentNumberOfColors")]
    pub current_number_of_colors: Option<u64>,

/// 
    #[serde(rename = "CurrentNumberOfColumns")]
    pub current_number_of_columns: Option<u32>,

/// 
    #[serde(rename = "CurrentNumberOfRows")]
    pub current_number_of_rows: Option<u32>,

/// 
    #[serde(rename = "CurrentRefreshRate")]
    pub current_refresh_rate: Option<u32>,

/// 
    #[serde(rename = "CurrentScanMode")]
    pub current_scan_mode: Option<u16>,

/// 
    #[serde(rename = "CurrentVerticalResolution")]
    pub current_vertical_resolution: Option<u32>,

/// 
    #[serde(rename = "MaxMemorySupported")]
    pub max_memory_supported: Option<u32>,

/// 
    #[serde(rename = "MaxRefreshRate")]
    pub max_refresh_rate: Option<u32>,

/// 
    #[serde(rename = "MinRefreshRate")]
    pub min_refresh_rate: Option<u32>,

/// 
    #[serde(rename = "NumberOfVideoPages")]
    pub number_of_video_pages: Option<u32>,

/// 
    #[serde(rename = "VideoMemoryType")]
    pub video_memory_type: Option<u16>,

/// 
    #[serde(rename = "VideoProcessor")]
    pub video_processor: Option<String>,
}

impl CIM_VideoController {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Controller::new(),
            accelerator_capabilities: Vec::new(),
            capability_descriptions: Vec::new(),
            current_bits_per_pixel: None,
            current_horizontal_resolution: None,
            current_number_of_colors: None,
            current_number_of_columns: None,
            current_number_of_rows: None,
            current_refresh_rate: None,
            current_scan_mode: None,
            current_vertical_resolution: None,
            max_memory_supported: None,
            max_refresh_rate: None,
            min_refresh_rate: None,
            number_of_video_pages: None,
            video_memory_type: None,
            video_processor: None,
        }
    }


    /// Sets the value of AcceleratorCapabilities
    pub fn set_accelerator_capabilities(&mut self, value: Vec<u16>) {
        self.accelerator_capabilities = value;
    }

    /// Gets the value of AcceleratorCapabilities
    pub fn get_accelerator_capabilities(&self) -> &Vec<u16> {
        &self.accelerator_capabilities
    }

    /// Sets the value of CapabilityDescriptions
    pub fn set_capability_descriptions(&mut self, value: Vec<String>) {
        self.capability_descriptions = value;
    }

    /// Gets the value of CapabilityDescriptions
    pub fn get_capability_descriptions(&self) -> &Vec<String> {
        &self.capability_descriptions
    }

    /// Sets the value of CurrentBitsPerPixel
    pub fn set_current_bits_per_pixel(&mut self, value: u32) {
        self.current_bits_per_pixel = Some(value);
    }

    /// Gets the value of CurrentBitsPerPixel
    pub fn get_current_bits_per_pixel(&self) -> Option<&u32> {
        self.current_bits_per_pixel.as_ref()
    }

    /// Sets the value of CurrentHorizontalResolution
    pub fn set_current_horizontal_resolution(&mut self, value: u32) {
        self.current_horizontal_resolution = Some(value);
    }

    /// Gets the value of CurrentHorizontalResolution
    pub fn get_current_horizontal_resolution(&self) -> Option<&u32> {
        self.current_horizontal_resolution.as_ref()
    }

    /// Sets the value of CurrentNumberOfColors
    pub fn set_current_number_of_colors(&mut self, value: u64) {
        self.current_number_of_colors = Some(value);
    }

    /// Gets the value of CurrentNumberOfColors
    pub fn get_current_number_of_colors(&self) -> Option<&u64> {
        self.current_number_of_colors.as_ref()
    }

    /// Sets the value of CurrentNumberOfColumns
    pub fn set_current_number_of_columns(&mut self, value: u32) {
        self.current_number_of_columns = Some(value);
    }

    /// Gets the value of CurrentNumberOfColumns
    pub fn get_current_number_of_columns(&self) -> Option<&u32> {
        self.current_number_of_columns.as_ref()
    }

    /// Sets the value of CurrentNumberOfRows
    pub fn set_current_number_of_rows(&mut self, value: u32) {
        self.current_number_of_rows = Some(value);
    }

    /// Gets the value of CurrentNumberOfRows
    pub fn get_current_number_of_rows(&self) -> Option<&u32> {
        self.current_number_of_rows.as_ref()
    }

    /// Sets the value of CurrentRefreshRate
    pub fn set_current_refresh_rate(&mut self, value: u32) {
        self.current_refresh_rate = Some(value);
    }

    /// Gets the value of CurrentRefreshRate
    pub fn get_current_refresh_rate(&self) -> Option<&u32> {
        self.current_refresh_rate.as_ref()
    }

    /// Sets the value of CurrentScanMode
    pub fn set_current_scan_mode(&mut self, value: u16) {
        self.current_scan_mode = Some(value);
    }

    /// Gets the value of CurrentScanMode
    pub fn get_current_scan_mode(&self) -> Option<&u16> {
        self.current_scan_mode.as_ref()
    }

    /// Sets the value of CurrentVerticalResolution
    pub fn set_current_vertical_resolution(&mut self, value: u32) {
        self.current_vertical_resolution = Some(value);
    }

    /// Gets the value of CurrentVerticalResolution
    pub fn get_current_vertical_resolution(&self) -> Option<&u32> {
        self.current_vertical_resolution.as_ref()
    }

    /// Sets the value of MaxMemorySupported
    pub fn set_max_memory_supported(&mut self, value: u32) {
        self.max_memory_supported = Some(value);
    }

    /// Gets the value of MaxMemorySupported
    pub fn get_max_memory_supported(&self) -> Option<&u32> {
        self.max_memory_supported.as_ref()
    }

    /// Sets the value of MaxRefreshRate
    pub fn set_max_refresh_rate(&mut self, value: u32) {
        self.max_refresh_rate = Some(value);
    }

    /// Gets the value of MaxRefreshRate
    pub fn get_max_refresh_rate(&self) -> Option<&u32> {
        self.max_refresh_rate.as_ref()
    }

    /// Sets the value of MinRefreshRate
    pub fn set_min_refresh_rate(&mut self, value: u32) {
        self.min_refresh_rate = Some(value);
    }

    /// Gets the value of MinRefreshRate
    pub fn get_min_refresh_rate(&self) -> Option<&u32> {
        self.min_refresh_rate.as_ref()
    }

    /// Sets the value of NumberOfVideoPages
    pub fn set_number_of_video_pages(&mut self, value: u32) {
        self.number_of_video_pages = Some(value);
    }

    /// Gets the value of NumberOfVideoPages
    pub fn get_number_of_video_pages(&self) -> Option<&u32> {
        self.number_of_video_pages.as_ref()
    }

    /// Sets the value of VideoMemoryType
    pub fn set_video_memory_type(&mut self, value: u16) {
        self.video_memory_type = Some(value);
    }

    /// Gets the value of VideoMemoryType
    pub fn get_video_memory_type(&self) -> Option<&u16> {
        self.video_memory_type.as_ref()
    }

    /// Sets the value of VideoProcessor
    pub fn set_video_processor(&mut self, value: String) {
        self.video_processor = Some(value);
    }

    /// Gets the value of VideoProcessor
    pub fn get_video_processor(&self) -> Option<&String> {
        self.video_processor.as_ref()
    }
}

