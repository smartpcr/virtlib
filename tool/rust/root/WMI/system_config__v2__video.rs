// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V2_Video struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V2_Video {
    #[serde(flatten)]
    pub base: SystemConfig_V2,

/// 
    #[serde(rename = "AdapterString")]
    pub adapter_string: Vec<char>,

/// 
    #[serde(rename = "BiosString")]
    pub bios_string: Vec<char>,

/// 
    #[serde(rename = "BitsPerPixel")]
    pub bits_per_pixel: Option<u32>,

/// 
    #[serde(rename = "ChipType")]
    pub chip_type: Vec<char>,

/// 
    #[serde(rename = "DACType")]
    pub dactype: Vec<char>,

/// 
    #[serde(rename = "DeviceId")]
    pub device_id: Vec<char>,

/// 
    #[serde(rename = "MemorySize")]
    pub memory_size: Option<u32>,

/// 
    #[serde(rename = "StateFlags")]
    pub state_flags: Option<u32>,

/// 
    #[serde(rename = "VRefresh")]
    pub vrefresh: Option<u32>,

/// 
    #[serde(rename = "XResolution")]
    pub xresolution: Option<u32>,

/// 
    #[serde(rename = "YResolution")]
    pub yresolution: Option<u32>,
}

impl SystemConfig_V2_Video {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V2::new(),
            adapter_string: Vec::new(),
            bios_string: Vec::new(),
            bits_per_pixel: None,
            chip_type: Vec::new(),
            dactype: Vec::new(),
            device_id: Vec::new(),
            memory_size: None,
            state_flags: None,
            vrefresh: None,
            xresolution: None,
            yresolution: None,
        }
    }


    /// Sets the value of AdapterString
    pub fn set_adapter_string(&mut self, value: Vec<char>) {
        self.adapter_string = value;
    }

    /// Gets the value of AdapterString
    pub fn get_adapter_string(&self) -> &Vec<char> {
        &self.adapter_string
    }

    /// Sets the value of BiosString
    pub fn set_bios_string(&mut self, value: Vec<char>) {
        self.bios_string = value;
    }

    /// Gets the value of BiosString
    pub fn get_bios_string(&self) -> &Vec<char> {
        &self.bios_string
    }

    /// Sets the value of BitsPerPixel
    pub fn set_bits_per_pixel(&mut self, value: u32) {
        self.bits_per_pixel = Some(value);
    }

    /// Gets the value of BitsPerPixel
    pub fn get_bits_per_pixel(&self) -> Option<&u32> {
        self.bits_per_pixel.as_ref()
    }

    /// Sets the value of ChipType
    pub fn set_chip_type(&mut self, value: Vec<char>) {
        self.chip_type = value;
    }

    /// Gets the value of ChipType
    pub fn get_chip_type(&self) -> &Vec<char> {
        &self.chip_type
    }

    /// Sets the value of DACType
    pub fn set_dactype(&mut self, value: Vec<char>) {
        self.dactype = value;
    }

    /// Gets the value of DACType
    pub fn get_dactype(&self) -> &Vec<char> {
        &self.dactype
    }

    /// Sets the value of DeviceId
    pub fn set_device_id(&mut self, value: Vec<char>) {
        self.device_id = value;
    }

    /// Gets the value of DeviceId
    pub fn get_device_id(&self) -> &Vec<char> {
        &self.device_id
    }

    /// Sets the value of MemorySize
    pub fn set_memory_size(&mut self, value: u32) {
        self.memory_size = Some(value);
    }

    /// Gets the value of MemorySize
    pub fn get_memory_size(&self) -> Option<&u32> {
        self.memory_size.as_ref()
    }

    /// Sets the value of StateFlags
    pub fn set_state_flags(&mut self, value: u32) {
        self.state_flags = Some(value);
    }

    /// Gets the value of StateFlags
    pub fn get_state_flags(&self) -> Option<&u32> {
        self.state_flags.as_ref()
    }

    /// Sets the value of VRefresh
    pub fn set_vrefresh(&mut self, value: u32) {
        self.vrefresh = Some(value);
    }

    /// Gets the value of VRefresh
    pub fn get_vrefresh(&self) -> Option<&u32> {
        self.vrefresh.as_ref()
    }

    /// Sets the value of XResolution
    pub fn set_xresolution(&mut self, value: u32) {
        self.xresolution = Some(value);
    }

    /// Gets the value of XResolution
    pub fn get_xresolution(&self) -> Option<&u32> {
        self.xresolution.as_ref()
    }

    /// Sets the value of YResolution
    pub fn set_yresolution(&mut self, value: u32) {
        self.yresolution = Some(value);
    }

    /// Gets the value of YResolution
    pub fn get_yresolution(&self) -> Option<&u32> {
        self.yresolution.as_ref()
    }
}

