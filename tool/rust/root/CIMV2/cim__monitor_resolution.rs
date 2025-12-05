// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_MonitorResolution struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_MonitorResolution {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "HorizontalResolution")]
    pub horizontal_resolution: Option<u32>,

/// 
    #[serde(rename = "MaxRefreshRate")]
    pub max_refresh_rate: Option<u32>,

/// 
    #[serde(rename = "MinRefreshRate")]
    pub min_refresh_rate: Option<u32>,

/// 
    #[serde(rename = "RefreshRate")]
    pub refresh_rate: Option<u32>,

/// 
    #[serde(rename = "ScanMode")]
    pub scan_mode: Option<u16>,

/// 
    #[serde(rename = "VerticalResolution")]
    pub vertical_resolution: Option<u32>,
}

impl CIM_MonitorResolution {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            horizontal_resolution: None,
            max_refresh_rate: None,
            min_refresh_rate: None,
            refresh_rate: None,
            scan_mode: None,
            vertical_resolution: None,
        }
    }


    /// Sets the value of HorizontalResolution
    pub fn set_horizontal_resolution(&mut self, value: u32) {
        self.horizontal_resolution = Some(value);
    }

    /// Gets the value of HorizontalResolution
    pub fn get_horizontal_resolution(&self) -> Option<&u32> {
        self.horizontal_resolution.as_ref()
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

    /// Sets the value of RefreshRate
    pub fn set_refresh_rate(&mut self, value: u32) {
        self.refresh_rate = Some(value);
    }

    /// Gets the value of RefreshRate
    pub fn get_refresh_rate(&self) -> Option<&u32> {
        self.refresh_rate.as_ref()
    }

    /// Sets the value of ScanMode
    pub fn set_scan_mode(&mut self, value: u16) {
        self.scan_mode = Some(value);
    }

    /// Gets the value of ScanMode
    pub fn get_scan_mode(&self) -> Option<&u16> {
        self.scan_mode.as_ref()
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

