// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_FlatPanel struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_FlatPanel {
    #[serde(flatten)]
    pub base: CIM_Display,

/// 
    #[serde(rename = "DisplayType")]
    pub display_type: Option<u16>,

/// 
    #[serde(rename = "HorizontalResolution")]
    pub horizontal_resolution: Option<u32>,

/// 
    #[serde(rename = "LightSource")]
    pub light_source: Option<u16>,

/// 
    #[serde(rename = "ScanMode")]
    pub scan_mode: Option<u16>,

/// 
    #[serde(rename = "SupportsColor")]
    pub supports_color: Option<bool>,

/// 
    #[serde(rename = "VerticalResolution")]
    pub vertical_resolution: Option<u32>,
}

impl CIM_FlatPanel {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Display::new(),
            display_type: None,
            horizontal_resolution: None,
            light_source: None,
            scan_mode: None,
            supports_color: None,
            vertical_resolution: None,
        }
    }


    /// Sets the value of DisplayType
    pub fn set_display_type(&mut self, value: u16) {
        self.display_type = Some(value);
    }

    /// Gets the value of DisplayType
    pub fn get_display_type(&self) -> Option<&u16> {
        self.display_type.as_ref()
    }

    /// Sets the value of HorizontalResolution
    pub fn set_horizontal_resolution(&mut self, value: u32) {
        self.horizontal_resolution = Some(value);
    }

    /// Gets the value of HorizontalResolution
    pub fn get_horizontal_resolution(&self) -> Option<&u32> {
        self.horizontal_resolution.as_ref()
    }

    /// Sets the value of LightSource
    pub fn set_light_source(&mut self, value: u16) {
        self.light_source = Some(value);
    }

    /// Gets the value of LightSource
    pub fn get_light_source(&self) -> Option<&u16> {
        self.light_source.as_ref()
    }

    /// Sets the value of ScanMode
    pub fn set_scan_mode(&mut self, value: u16) {
        self.scan_mode = Some(value);
    }

    /// Gets the value of ScanMode
    pub fn get_scan_mode(&self) -> Option<&u16> {
        self.scan_mode.as_ref()
    }

    /// Sets the value of SupportsColor
    pub fn set_supports_color(&mut self, value: bool) {
        self.supports_color = Some(value);
    }

    /// Gets the value of SupportsColor
    pub fn get_supports_color(&self) -> Option<&bool> {
        self.supports_color.as_ref()
    }

    /// Sets the value of VerticalResolution
    pub fn set_vertical_resolution(&mut self, value: u32) {
        self.vertical_resolution = Some(value);
    }

    /// Gets the value of VerticalResolution
    pub fn get_vertical_resolution(&self) -> Option<&u32> {
        self.vertical_resolution.as_ref()
    }
}

