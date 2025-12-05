// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_DesktopMonitor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_DesktopMonitor {
    #[serde(flatten)]
    pub base: CIM_Display,

/// 
    #[serde(rename = "Bandwidth")]
    pub bandwidth: Option<u32>,

/// 
    #[serde(rename = "DisplayType")]
    pub display_type: Option<u16>,

/// 
    #[serde(rename = "ScreenHeight")]
    pub screen_height: Option<u32>,

/// 
    #[serde(rename = "ScreenWidth")]
    pub screen_width: Option<u32>,
}

impl CIM_DesktopMonitor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Display::new(),
            bandwidth: None,
            display_type: None,
            screen_height: None,
            screen_width: None,
        }
    }


    /// Sets the value of Bandwidth
    pub fn set_bandwidth(&mut self, value: u32) {
        self.bandwidth = Some(value);
    }

    /// Gets the value of Bandwidth
    pub fn get_bandwidth(&self) -> Option<&u32> {
        self.bandwidth.as_ref()
    }

    /// Sets the value of DisplayType
    pub fn set_display_type(&mut self, value: u16) {
        self.display_type = Some(value);
    }

    /// Gets the value of DisplayType
    pub fn get_display_type(&self) -> Option<&u16> {
        self.display_type.as_ref()
    }

    /// Sets the value of ScreenHeight
    pub fn set_screen_height(&mut self, value: u32) {
        self.screen_height = Some(value);
    }

    /// Gets the value of ScreenHeight
    pub fn get_screen_height(&self) -> Option<&u32> {
        self.screen_height.as_ref()
    }

    /// Sets the value of ScreenWidth
    pub fn set_screen_width(&mut self, value: u32) {
        self.screen_width = Some(value);
    }

    /// Gets the value of ScreenWidth
    pub fn get_screen_width(&self) -> Option<&u32> {
        self.screen_width.as_ref()
    }
}

