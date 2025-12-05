// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_DisplayControllerConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_DisplayControllerConfiguration {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "BitsPerPixel")]
    pub bits_per_pixel: Option<u32>,

/// 
    #[serde(rename = "ColorPlanes")]
    pub color_planes: Option<u32>,

/// 
    #[serde(rename = "DeviceEntriesInAColorTable")]
    pub device_entries_in_acolor_table: Option<u32>,

/// 
    #[serde(rename = "DeviceSpecificPens")]
    pub device_specific_pens: Option<u32>,

/// 
    #[serde(rename = "HorizontalResolution")]
    pub horizontal_resolution: Option<u32>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "RefreshRate")]
    pub refresh_rate: Option<i32>,

/// 
    #[serde(rename = "ReservedSystemPaletteEntries")]
    pub reserved_system_palette_entries: Option<u32>,

/// 
    #[serde(rename = "SystemPaletteEntries")]
    pub system_palette_entries: Option<u32>,

/// 
    #[serde(rename = "VerticalResolution")]
    pub vertical_resolution: Option<u32>,

/// 
    #[serde(rename = "VideoMode")]
    pub video_mode: Option<String>,
}

impl Win32_DisplayControllerConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            bits_per_pixel: None,
            color_planes: None,
            device_entries_in_acolor_table: None,
            device_specific_pens: None,
            horizontal_resolution: None,
            name: None,
            refresh_rate: None,
            reserved_system_palette_entries: None,
            system_palette_entries: None,
            vertical_resolution: None,
            video_mode: None,
        }
    }


    /// Sets the value of BitsPerPixel
    pub fn set_bits_per_pixel(&mut self, value: u32) {
        self.bits_per_pixel = Some(value);
    }

    /// Gets the value of BitsPerPixel
    pub fn get_bits_per_pixel(&self) -> Option<&u32> {
        self.bits_per_pixel.as_ref()
    }

    /// Sets the value of ColorPlanes
    pub fn set_color_planes(&mut self, value: u32) {
        self.color_planes = Some(value);
    }

    /// Gets the value of ColorPlanes
    pub fn get_color_planes(&self) -> Option<&u32> {
        self.color_planes.as_ref()
    }

    /// Sets the value of DeviceEntriesInAColorTable
    pub fn set_device_entries_in_acolor_table(&mut self, value: u32) {
        self.device_entries_in_acolor_table = Some(value);
    }

    /// Gets the value of DeviceEntriesInAColorTable
    pub fn get_device_entries_in_acolor_table(&self) -> Option<&u32> {
        self.device_entries_in_acolor_table.as_ref()
    }

    /// Sets the value of DeviceSpecificPens
    pub fn set_device_specific_pens(&mut self, value: u32) {
        self.device_specific_pens = Some(value);
    }

    /// Gets the value of DeviceSpecificPens
    pub fn get_device_specific_pens(&self) -> Option<&u32> {
        self.device_specific_pens.as_ref()
    }

    /// Sets the value of HorizontalResolution
    pub fn set_horizontal_resolution(&mut self, value: u32) {
        self.horizontal_resolution = Some(value);
    }

    /// Gets the value of HorizontalResolution
    pub fn get_horizontal_resolution(&self) -> Option<&u32> {
        self.horizontal_resolution.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of RefreshRate
    pub fn set_refresh_rate(&mut self, value: i32) {
        self.refresh_rate = Some(value);
    }

    /// Gets the value of RefreshRate
    pub fn get_refresh_rate(&self) -> Option<&i32> {
        self.refresh_rate.as_ref()
    }

    /// Sets the value of ReservedSystemPaletteEntries
    pub fn set_reserved_system_palette_entries(&mut self, value: u32) {
        self.reserved_system_palette_entries = Some(value);
    }

    /// Gets the value of ReservedSystemPaletteEntries
    pub fn get_reserved_system_palette_entries(&self) -> Option<&u32> {
        self.reserved_system_palette_entries.as_ref()
    }

    /// Sets the value of SystemPaletteEntries
    pub fn set_system_palette_entries(&mut self, value: u32) {
        self.system_palette_entries = Some(value);
    }

    /// Gets the value of SystemPaletteEntries
    pub fn get_system_palette_entries(&self) -> Option<&u32> {
        self.system_palette_entries.as_ref()
    }

    /// Sets the value of VerticalResolution
    pub fn set_vertical_resolution(&mut self, value: u32) {
        self.vertical_resolution = Some(value);
    }

    /// Gets the value of VerticalResolution
    pub fn get_vertical_resolution(&self) -> Option<&u32> {
        self.vertical_resolution.as_ref()
    }

    /// Sets the value of VideoMode
    pub fn set_video_mode(&mut self, value: String) {
        self.video_mode = Some(value);
    }

    /// Gets the value of VideoMode
    pub fn get_video_mode(&self) -> Option<&String> {
        self.video_mode.as_ref()
    }
}

