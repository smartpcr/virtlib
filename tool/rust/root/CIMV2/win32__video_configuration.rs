// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_VideoConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_VideoConfiguration {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "ActualColorResolution")]
    pub actual_color_resolution: Option<u32>,

/// 
    #[serde(rename = "AdapterChipType")]
    pub adapter_chip_type: Option<String>,

/// 
    #[serde(rename = "AdapterCompatibility")]
    pub adapter_compatibility: Option<String>,

/// 
    #[serde(rename = "AdapterDACType")]
    pub adapter_dactype: Option<String>,

/// 
    #[serde(rename = "AdapterDescription")]
    pub adapter_description: Option<String>,

/// 
    #[serde(rename = "AdapterRAM")]
    pub adapter_ram: Option<u32>,

/// 
    #[serde(rename = "AdapterType")]
    pub adapter_type: Option<String>,

/// 
    #[serde(rename = "BitsPerPixel")]
    pub bits_per_pixel: Option<u32>,

/// 
    #[serde(rename = "ColorPlanes")]
    pub color_planes: Option<u32>,

/// 
    #[serde(rename = "ColorTableEntries")]
    pub color_table_entries: Option<u32>,

/// 
    #[serde(rename = "DeviceSpecificPens")]
    pub device_specific_pens: Option<u32>,

/// 
    #[serde(rename = "DriverDate")]
    pub driver_date: Option<String>,

/// 
    #[serde(rename = "HorizontalResolution")]
    pub horizontal_resolution: Option<u32>,

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
    #[serde(rename = "MonitorManufacturer")]
    pub monitor_manufacturer: Option<String>,

/// 
    #[serde(rename = "MonitorType")]
    pub monitor_type: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "PixelsPerXLogicalInch")]
    pub pixels_per_xlogical_inch: Option<u32>,

/// 
    #[serde(rename = "PixelsPerYLogicalInch")]
    pub pixels_per_ylogical_inch: Option<u32>,

/// 
    #[serde(rename = "RefreshRate")]
    pub refresh_rate: Option<u32>,

/// 
    #[serde(rename = "ScanMode")]
    pub scan_mode: Option<String>,

/// 
    #[serde(rename = "ScreenHeight")]
    pub screen_height: Option<u32>,

/// 
    #[serde(rename = "ScreenWidth")]
    pub screen_width: Option<u32>,

/// 
    #[serde(rename = "SystemPaletteEntries")]
    pub system_palette_entries: Option<u32>,

/// 
    #[serde(rename = "VerticalResolution")]
    pub vertical_resolution: Option<u32>,
}

impl Win32_VideoConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            actual_color_resolution: None,
            adapter_chip_type: None,
            adapter_compatibility: None,
            adapter_dactype: None,
            adapter_description: None,
            adapter_ram: None,
            adapter_type: None,
            bits_per_pixel: None,
            color_planes: None,
            color_table_entries: None,
            device_specific_pens: None,
            driver_date: None,
            horizontal_resolution: None,
            inf_filename: None,
            inf_section: None,
            installed_display_drivers: None,
            monitor_manufacturer: None,
            monitor_type: None,
            name: None,
            pixels_per_xlogical_inch: None,
            pixels_per_ylogical_inch: None,
            refresh_rate: None,
            scan_mode: None,
            screen_height: None,
            screen_width: None,
            system_palette_entries: None,
            vertical_resolution: None,
        }
    }


    /// Sets the value of ActualColorResolution
    pub fn set_actual_color_resolution(&mut self, value: u32) {
        self.actual_color_resolution = Some(value);
    }

    /// Gets the value of ActualColorResolution
    pub fn get_actual_color_resolution(&self) -> Option<&u32> {
        self.actual_color_resolution.as_ref()
    }

    /// Sets the value of AdapterChipType
    pub fn set_adapter_chip_type(&mut self, value: String) {
        self.adapter_chip_type = Some(value);
    }

    /// Gets the value of AdapterChipType
    pub fn get_adapter_chip_type(&self) -> Option<&String> {
        self.adapter_chip_type.as_ref()
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

    /// Sets the value of AdapterDescription
    pub fn set_adapter_description(&mut self, value: String) {
        self.adapter_description = Some(value);
    }

    /// Gets the value of AdapterDescription
    pub fn get_adapter_description(&self) -> Option<&String> {
        self.adapter_description.as_ref()
    }

    /// Sets the value of AdapterRAM
    pub fn set_adapter_ram(&mut self, value: u32) {
        self.adapter_ram = Some(value);
    }

    /// Gets the value of AdapterRAM
    pub fn get_adapter_ram(&self) -> Option<&u32> {
        self.adapter_ram.as_ref()
    }

    /// Sets the value of AdapterType
    pub fn set_adapter_type(&mut self, value: String) {
        self.adapter_type = Some(value);
    }

    /// Gets the value of AdapterType
    pub fn get_adapter_type(&self) -> Option<&String> {
        self.adapter_type.as_ref()
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

    /// Sets the value of DriverDate
    pub fn set_driver_date(&mut self, value: String) {
        self.driver_date = Some(value);
    }

    /// Gets the value of DriverDate
    pub fn get_driver_date(&self) -> Option<&String> {
        self.driver_date.as_ref()
    }

    /// Sets the value of HorizontalResolution
    pub fn set_horizontal_resolution(&mut self, value: u32) {
        self.horizontal_resolution = Some(value);
    }

    /// Gets the value of HorizontalResolution
    pub fn get_horizontal_resolution(&self) -> Option<&u32> {
        self.horizontal_resolution.as_ref()
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

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
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

    /// Sets the value of RefreshRate
    pub fn set_refresh_rate(&mut self, value: u32) {
        self.refresh_rate = Some(value);
    }

    /// Gets the value of RefreshRate
    pub fn get_refresh_rate(&self) -> Option<&u32> {
        self.refresh_rate.as_ref()
    }

    /// Sets the value of ScanMode
    pub fn set_scan_mode(&mut self, value: String) {
        self.scan_mode = Some(value);
    }

    /// Gets the value of ScanMode
    pub fn get_scan_mode(&self) -> Option<&String> {
        self.scan_mode.as_ref()
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
}

