// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WmiMonitorSupportedDisplayFeatures struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WmiMonitorSupportedDisplayFeatures {

/// 
    #[serde(rename = "ActiveOffSupported")]
    pub active_off_supported: Option<bool>,

/// 
    #[serde(rename = "DisplayType")]
    pub display_type: Option<u8>,

/// 
    #[serde(rename = "GTFSupported")]
    pub gtfsupported: Option<bool>,

/// 
    #[serde(rename = "HasPreferredTimingMode")]
    pub has_preferred_timing_mode: Option<bool>,

/// 
    #[serde(rename = "sRGBSupported")]
    pub s_rgbsupported: Option<bool>,

/// 
    #[serde(rename = "StandbySupported")]
    pub standby_supported: Option<bool>,

/// 
    #[serde(rename = "SuspendSupported")]
    pub suspend_supported: Option<bool>,
}

impl WmiMonitorSupportedDisplayFeatures {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active_off_supported: None,
            display_type: None,
            gtfsupported: None,
            has_preferred_timing_mode: None,
            s_rgbsupported: None,
            standby_supported: None,
            suspend_supported: None,
        }
    }


    /// Sets the value of ActiveOffSupported
    pub fn set_active_off_supported(&mut self, value: bool) {
        self.active_off_supported = Some(value);
    }

    /// Gets the value of ActiveOffSupported
    pub fn get_active_off_supported(&self) -> Option<&bool> {
        self.active_off_supported.as_ref()
    }

    /// Sets the value of DisplayType
    pub fn set_display_type(&mut self, value: u8) {
        self.display_type = Some(value);
    }

    /// Gets the value of DisplayType
    pub fn get_display_type(&self) -> Option<&u8> {
        self.display_type.as_ref()
    }

    /// Sets the value of GTFSupported
    pub fn set_gtfsupported(&mut self, value: bool) {
        self.gtfsupported = Some(value);
    }

    /// Gets the value of GTFSupported
    pub fn get_gtfsupported(&self) -> Option<&bool> {
        self.gtfsupported.as_ref()
    }

    /// Sets the value of HasPreferredTimingMode
    pub fn set_has_preferred_timing_mode(&mut self, value: bool) {
        self.has_preferred_timing_mode = Some(value);
    }

    /// Gets the value of HasPreferredTimingMode
    pub fn get_has_preferred_timing_mode(&self) -> Option<&bool> {
        self.has_preferred_timing_mode.as_ref()
    }

    /// Sets the value of sRGBSupported
    pub fn set_s_rgbsupported(&mut self, value: bool) {
        self.s_rgbsupported = Some(value);
    }

    /// Gets the value of sRGBSupported
    pub fn get_s_rgbsupported(&self) -> Option<&bool> {
        self.s_rgbsupported.as_ref()
    }

    /// Sets the value of StandbySupported
    pub fn set_standby_supported(&mut self, value: bool) {
        self.standby_supported = Some(value);
    }

    /// Gets the value of StandbySupported
    pub fn get_standby_supported(&self) -> Option<&bool> {
        self.standby_supported.as_ref()
    }

    /// Sets the value of SuspendSupported
    pub fn set_suspend_supported(&mut self, value: bool) {
        self.suspend_supported = Some(value);
    }

    /// Gets the value of SuspendSupported
    pub fn get_suspend_supported(&self) -> Option<&bool> {
        self.suspend_supported.as_ref()
    }
}

