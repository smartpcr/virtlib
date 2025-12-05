// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_DisplayConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_DisplayConfiguration {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "BitsPerPel")]
    pub bits_per_pel: Option<u32>,

/// 
    #[serde(rename = "DeviceName")]
    pub device_name: Option<String>,

/// 
    #[serde(rename = "DisplayFlags")]
    pub display_flags: Option<u32>,

/// 
    #[serde(rename = "DisplayFrequency")]
    pub display_frequency: Option<u32>,

/// 
    #[serde(rename = "DitherType")]
    pub dither_type: Option<u32>,

/// 
    #[serde(rename = "DriverVersion")]
    pub driver_version: Option<String>,

/// 
    #[serde(rename = "ICMIntent")]
    pub icmintent: Option<u32>,

/// 
    #[serde(rename = "ICMMethod")]
    pub icmmethod: Option<u32>,

/// 
    #[serde(rename = "LogPixels")]
    pub log_pixels: Option<u32>,

/// 
    #[serde(rename = "PelsHeight")]
    pub pels_height: Option<u32>,

/// 
    #[serde(rename = "PelsWidth")]
    pub pels_width: Option<u32>,

/// 
    #[serde(rename = "SpecificationVersion")]
    pub specification_version: Option<u32>,
}

impl Win32_DisplayConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            bits_per_pel: None,
            device_name: None,
            display_flags: None,
            display_frequency: None,
            dither_type: None,
            driver_version: None,
            icmintent: None,
            icmmethod: None,
            log_pixels: None,
            pels_height: None,
            pels_width: None,
            specification_version: None,
        }
    }


    /// Sets the value of BitsPerPel
    pub fn set_bits_per_pel(&mut self, value: u32) {
        self.bits_per_pel = Some(value);
    }

    /// Gets the value of BitsPerPel
    pub fn get_bits_per_pel(&self) -> Option<&u32> {
        self.bits_per_pel.as_ref()
    }

    /// Sets the value of DeviceName
    pub fn set_device_name(&mut self, value: String) {
        self.device_name = Some(value);
    }

    /// Gets the value of DeviceName
    pub fn get_device_name(&self) -> Option<&String> {
        self.device_name.as_ref()
    }

    /// Sets the value of DisplayFlags
    pub fn set_display_flags(&mut self, value: u32) {
        self.display_flags = Some(value);
    }

    /// Gets the value of DisplayFlags
    pub fn get_display_flags(&self) -> Option<&u32> {
        self.display_flags.as_ref()
    }

    /// Sets the value of DisplayFrequency
    pub fn set_display_frequency(&mut self, value: u32) {
        self.display_frequency = Some(value);
    }

    /// Gets the value of DisplayFrequency
    pub fn get_display_frequency(&self) -> Option<&u32> {
        self.display_frequency.as_ref()
    }

    /// Sets the value of DitherType
    pub fn set_dither_type(&mut self, value: u32) {
        self.dither_type = Some(value);
    }

    /// Gets the value of DitherType
    pub fn get_dither_type(&self) -> Option<&u32> {
        self.dither_type.as_ref()
    }

    /// Sets the value of DriverVersion
    pub fn set_driver_version(&mut self, value: String) {
        self.driver_version = Some(value);
    }

    /// Gets the value of DriverVersion
    pub fn get_driver_version(&self) -> Option<&String> {
        self.driver_version.as_ref()
    }

    /// Sets the value of ICMIntent
    pub fn set_icmintent(&mut self, value: u32) {
        self.icmintent = Some(value);
    }

    /// Gets the value of ICMIntent
    pub fn get_icmintent(&self) -> Option<&u32> {
        self.icmintent.as_ref()
    }

    /// Sets the value of ICMMethod
    pub fn set_icmmethod(&mut self, value: u32) {
        self.icmmethod = Some(value);
    }

    /// Gets the value of ICMMethod
    pub fn get_icmmethod(&self) -> Option<&u32> {
        self.icmmethod.as_ref()
    }

    /// Sets the value of LogPixels
    pub fn set_log_pixels(&mut self, value: u32) {
        self.log_pixels = Some(value);
    }

    /// Gets the value of LogPixels
    pub fn get_log_pixels(&self) -> Option<&u32> {
        self.log_pixels.as_ref()
    }

    /// Sets the value of PelsHeight
    pub fn set_pels_height(&mut self, value: u32) {
        self.pels_height = Some(value);
    }

    /// Gets the value of PelsHeight
    pub fn get_pels_height(&self) -> Option<&u32> {
        self.pels_height.as_ref()
    }

    /// Sets the value of PelsWidth
    pub fn set_pels_width(&mut self, value: u32) {
        self.pels_width = Some(value);
    }

    /// Gets the value of PelsWidth
    pub fn get_pels_width(&self) -> Option<&u32> {
        self.pels_width.as_ref()
    }

    /// Sets the value of SpecificationVersion
    pub fn set_specification_version(&mut self, value: u32) {
        self.specification_version = Some(value);
    }

    /// Gets the value of SpecificationVersion
    pub fn get_specification_version(&self) -> Option<&u32> {
        self.specification_version.as_ref()
    }
}

