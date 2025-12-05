// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_VideoController struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_VideoController {
    #[serde(flatten)]
    pub base: CIM_PCVideoController,

/// 
    #[serde(rename = "AdapterCompatibility")]
    pub adapter_compatibility: Option<String>,

/// 
    #[serde(rename = "AdapterDACType")]
    pub adapter_dactype: Option<String>,

/// 
    #[serde(rename = "AdapterRAM")]
    pub adapter_ram: Option<u32>,

/// 
    #[serde(rename = "ColorTableEntries")]
    pub color_table_entries: Option<u32>,

/// 
    #[serde(rename = "DeviceSpecificPens")]
    pub device_specific_pens: Option<u32>,

/// 
    #[serde(rename = "DitherType")]
    pub dither_type: Option<u32>,

/// 
    #[serde(rename = "DriverDate")]
    pub driver_date: Option<String>,

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
    #[serde(rename = "InfFilename")]
    pub inf_filename: Option<String>,

/// 
    #[serde(rename = "InfSection")]
    pub inf_section: Option<String>,

/// 
    #[serde(rename = "InstalledDisplayDrivers")]
    pub installed_display_drivers: Option<String>,

/// 
    #[serde(rename = "Monochrome")]
    pub monochrome: Option<bool>,

/// 
    #[serde(rename = "ReservedSystemPaletteEntries")]
    pub reserved_system_palette_entries: Option<u32>,

/// 
    #[serde(rename = "SpecificationVersion")]
    pub specification_version: Option<u32>,

/// 
    #[serde(rename = "SystemPaletteEntries")]
    pub system_palette_entries: Option<u32>,

/// 
    #[serde(rename = "VideoModeDescription")]
    pub video_mode_description: Option<String>,
}

impl Win32_VideoController {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PCVideoController::new(),
            adapter_compatibility: None,
            adapter_dactype: None,
            adapter_ram: None,
            color_table_entries: None,
            device_specific_pens: None,
            dither_type: None,
            driver_date: None,
            driver_version: None,
            icmintent: None,
            icmmethod: None,
            inf_filename: None,
            inf_section: None,
            installed_display_drivers: None,
            monochrome: None,
            reserved_system_palette_entries: None,
            specification_version: None,
            system_palette_entries: None,
            video_mode_description: None,
        }
    }


    /// Sets the value of AdapterCompatibility
    pub fn set_adapter_compatibility(&mut self, value: String) {
        self.adapter_compatibility = Some(value);
    }

    /// Gets the value of AdapterCompatibility
    pub fn get_adapter_compatibility(&self) -> Option<&String> {
        self.adapter_compatibility.as_ref()
    }

    /// Sets the value of AdapterDACType
    pub fn set_adapter_dactype(&mut self, value: String) {
        self.adapter_dactype = Some(value);
    }

    /// Gets the value of AdapterDACType
    pub fn get_adapter_dactype(&self) -> Option<&String> {
        self.adapter_dactype.as_ref()
    }

    /// Sets the value of AdapterRAM
    pub fn set_adapter_ram(&mut self, value: u32) {
        self.adapter_ram = Some(value);
    }

    /// Gets the value of AdapterRAM
    pub fn get_adapter_ram(&self) -> Option<&u32> {
        self.adapter_ram.as_ref()
    }

    /// Sets the value of ColorTableEntries
    pub fn set_color_table_entries(&mut self, value: u32) {
        self.color_table_entries = Some(value);
    }

    /// Gets the value of ColorTableEntries
    pub fn get_color_table_entries(&self) -> Option<&u32> {
        self.color_table_entries.as_ref()
    }

    /// Sets the value of DeviceSpecificPens
    pub fn set_device_specific_pens(&mut self, value: u32) {
        self.device_specific_pens = Some(value);
    }

    /// Gets the value of DeviceSpecificPens
    pub fn get_device_specific_pens(&self) -> Option<&u32> {
        self.device_specific_pens.as_ref()
    }

    /// Sets the value of DitherType
    pub fn set_dither_type(&mut self, value: u32) {
        self.dither_type = Some(value);
    }

    /// Gets the value of DitherType
    pub fn get_dither_type(&self) -> Option<&u32> {
        self.dither_type.as_ref()
    }

    /// Sets the value of DriverDate
    pub fn set_driver_date(&mut self, value: String) {
        self.driver_date = Some(value);
    }

    /// Gets the value of DriverDate
    pub fn get_driver_date(&self) -> Option<&String> {
        self.driver_date.as_ref()
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

    /// Sets the value of InfFilename
    pub fn set_inf_filename(&mut self, value: String) {
        self.inf_filename = Some(value);
    }

    /// Gets the value of InfFilename
    pub fn get_inf_filename(&self) -> Option<&String> {
        self.inf_filename.as_ref()
    }

    /// Sets the value of InfSection
    pub fn set_inf_section(&mut self, value: String) {
        self.inf_section = Some(value);
    }

    /// Gets the value of InfSection
    pub fn get_inf_section(&self) -> Option<&String> {
        self.inf_section.as_ref()
    }

    /// Sets the value of InstalledDisplayDrivers
    pub fn set_installed_display_drivers(&mut self, value: String) {
        self.installed_display_drivers = Some(value);
    }

    /// Gets the value of InstalledDisplayDrivers
    pub fn get_installed_display_drivers(&self) -> Option<&String> {
        self.installed_display_drivers.as_ref()
    }

    /// Sets the value of Monochrome
    pub fn set_monochrome(&mut self, value: bool) {
        self.monochrome = Some(value);
    }

    /// Gets the value of Monochrome
    pub fn get_monochrome(&self) -> Option<&bool> {
        self.monochrome.as_ref()
    }

    /// Sets the value of ReservedSystemPaletteEntries
    pub fn set_reserved_system_palette_entries(&mut self, value: u32) {
        self.reserved_system_palette_entries = Some(value);
    }

    /// Gets the value of ReservedSystemPaletteEntries
    pub fn get_reserved_system_palette_entries(&self) -> Option<&u32> {
        self.reserved_system_palette_entries.as_ref()
    }

    /// Sets the value of SpecificationVersion
    pub fn set_specification_version(&mut self, value: u32) {
        self.specification_version = Some(value);
    }

    /// Gets the value of SpecificationVersion
    pub fn get_specification_version(&self) -> Option<&u32> {
        self.specification_version.as_ref()
    }

    /// Sets the value of SystemPaletteEntries
    pub fn set_system_palette_entries(&mut self, value: u32) {
        self.system_palette_entries = Some(value);
    }

    /// Gets the value of SystemPaletteEntries
    pub fn get_system_palette_entries(&self) -> Option<&u32> {
        self.system_palette_entries.as_ref()
    }

    /// Sets the value of VideoModeDescription
    pub fn set_video_mode_description(&mut self, value: String) {
        self.video_mode_description = Some(value);
    }

    /// Gets the value of VideoModeDescription
    pub fn get_video_mode_description(&self) -> Option<&String> {
        self.video_mode_description.as_ref()
    }
}

