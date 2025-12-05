// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_VideoHead struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_VideoHead {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// The number of bits used to display each pixel.
    #[serde(rename = "CurrentBitsPerPixel")]
    pub current_bits_per_pixel: Option<u32>,

/// Current number of horizontal pixels.
    #[serde(rename = "CurrentHorizontalResolution")]
    pub current_horizontal_resolution: Option<u32>,

/// Number of colors supported at the current resolutions.
    #[serde(rename = "CurrentNumberOfColors")]
    pub current_number_of_colors: Option<u64>,

/// If in character mode, number of columns for this DisplayController. Otherwise, enter 0.
    #[serde(rename = "CurrentNumberOfColumns")]
    pub current_number_of_columns: Option<u32>,

/// If in character mode, number of rows for this Video Controller. Otherwise, enter 0.
    #[serde(rename = "CurrentNumberOfRows")]
    pub current_number_of_rows: Option<u32>,

/// Current refresh rate in Hertz.
    #[serde(rename = "CurrentRefreshRate")]
    pub current_refresh_rate: Option<u32>,

/// Current scan mode.
    #[serde(rename = "CurrentScanMode")]
    pub current_scan_mode: Option<VideoHead_CurrentScanMode>,

/// Current number of vertical pixels.
    #[serde(rename = "CurrentVerticalResolution")]
    pub current_vertical_resolution: Option<u32>,

/// Maximum refresh rate of the DisplayController in Hertz.
    #[serde(rename = "MaxRefreshRate")]
    pub max_refresh_rate: Option<u32>,

/// Minimum refresh rate of the Video Controller in Hertz.
    #[serde(rename = "MinRefreshRate")]
    pub min_refresh_rate: Option<u32>,

/// A string describing the current scan mode when the instance's CurrentScanMode property is 1 ("Other").
    #[serde(rename = "OtherCurrentScanMode")]
    pub other_current_scan_mode: Option<String>,
}

impl CIM_VideoHead {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            current_bits_per_pixel: None,
            current_horizontal_resolution: None,
            current_number_of_colors: None,
            current_number_of_columns: None,
            current_number_of_rows: None,
            current_refresh_rate: None,
            current_scan_mode: None,
            current_vertical_resolution: None,
            max_refresh_rate: None,
            min_refresh_rate: None,
            other_current_scan_mode: None,
        }
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
    pub fn set_current_scan_mode(&mut self, value: VideoHead_CurrentScanMode) {
        self.current_scan_mode = Some(value);
    }

    /// Gets the value of CurrentScanMode
    pub fn get_current_scan_mode(&self) -> Option<&VideoHead_CurrentScanMode> {
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

    /// Sets the value of OtherCurrentScanMode
    pub fn set_other_current_scan_mode(&mut self, value: String) {
        self.other_current_scan_mode = Some(value);
    }

    /// Gets the value of OtherCurrentScanMode
    pub fn get_other_current_scan_mode(&self) -> Option<&String> {
        self.other_current_scan_mode.as_ref()
    }
}

