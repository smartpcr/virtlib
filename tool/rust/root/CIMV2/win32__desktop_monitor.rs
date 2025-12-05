// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_DesktopMonitor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_DesktopMonitor {
    #[serde(flatten)]
    pub base: CIM_DesktopMonitor,

/// 
    #[serde(rename = "MonitorManufacturer")]
    pub monitor_manufacturer: Option<String>,

/// 
    #[serde(rename = "MonitorType")]
    pub monitor_type: Option<String>,

/// 
    #[serde(rename = "PixelsPerXLogicalInch")]
    pub pixels_per_xlogical_inch: Option<u32>,

/// 
    #[serde(rename = "PixelsPerYLogicalInch")]
    pub pixels_per_ylogical_inch: Option<u32>,
}

impl Win32_DesktopMonitor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_DesktopMonitor::new(),
            monitor_manufacturer: None,
            monitor_type: None,
            pixels_per_xlogical_inch: None,
            pixels_per_ylogical_inch: None,
        }
    }


    /// Sets the value of MonitorManufacturer
    pub fn set_monitor_manufacturer(&mut self, value: String) {
        self.monitor_manufacturer = Some(value);
    }

    /// Gets the value of MonitorManufacturer
    pub fn get_monitor_manufacturer(&self) -> Option<&String> {
        self.monitor_manufacturer.as_ref()
    }

    /// Sets the value of MonitorType
    pub fn set_monitor_type(&mut self, value: String) {
        self.monitor_type = Some(value);
    }

    /// Gets the value of MonitorType
    pub fn get_monitor_type(&self) -> Option<&String> {
        self.monitor_type.as_ref()
    }

    /// Sets the value of PixelsPerXLogicalInch
    pub fn set_pixels_per_xlogical_inch(&mut self, value: u32) {
        self.pixels_per_xlogical_inch = Some(value);
    }

    /// Gets the value of PixelsPerXLogicalInch
    pub fn get_pixels_per_xlogical_inch(&self) -> Option<&u32> {
        self.pixels_per_xlogical_inch.as_ref()
    }

    /// Sets the value of PixelsPerYLogicalInch
    pub fn set_pixels_per_ylogical_inch(&mut self, value: u32) {
        self.pixels_per_ylogical_inch = Some(value);
    }

    /// Gets the value of PixelsPerYLogicalInch
    pub fn get_pixels_per_ylogical_inch(&self) -> Option<&u32> {
        self.pixels_per_ylogical_inch.as_ref()
    }
}

